//! Session state management for semantic code editing operations.
//!
//! This module provides session management, file caching, and operation staging
//! functionality for the semantic code editor. It manages persistent state
//! across editing operations and provides configurable performance optimizations.
//!
//! ## Key Components
//!
//! - [`SemanticEditTools`]: Main state container with session and cache management
//! - [`StagedOperation`]: Represents an operation that can be previewed and committed
//! - [`CacheStats`]: Performance statistics for file caching
//! - [`StatsLruCache`]: LRU cache with performance tracking
//!
//! ## Features
//!
//! - **Session isolation**: Separate contexts for different projects
//! - **File caching**: Configurable LRU cache with performance statistics
//! - **Operation staging**: Preview changes before applying them
//! - **Path resolution**: Context-aware path handling (relative/absolute)
//! - **Performance monitoring**: Cache hit/miss tracking and reporting

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use fieldwork::Fieldwork;
use lru::LruCache;
use serde::{Deserialize, Serialize};

use crate::editor::EditPosition;
use crate::error::SemanticEditError;
use crate::filesystem::{FileOperations, StdFileOperations};
use crate::languages::{LanguageName, LanguageRegistry};
use crate::selector::Selector;
use mcplease::session::SessionStore;

/// Cache performance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub total_requests: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.hits as f64 / self.total_requests as f64
        }
    }
}

/// LRU cache wrapper that tracks statistics
#[derive(Debug)]
pub struct StatsLruCache {
    cache: LruCache<String, String>,
    stats: CacheStats,
}

impl StatsLruCache {
    pub fn new(cap: NonZeroUsize) -> Self {
        Self {
            cache: LruCache::new(cap),
            stats: CacheStats::default(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.stats.total_requests += 1;
        match self.cache.get(key) {
            Some(value) => {
                self.stats.hits += 1;
                Some(value)
            }
            None => {
                self.stats.misses += 1;
                None
            }
        }
    }

    pub fn put(&mut self, key: String, value: String) -> Option<String> {
        self.cache.put(key, value)
    }

    pub fn cap(&self) -> NonZeroUsize {
        self.cache.cap()
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    pub fn clear_stats(&mut self) {
        self.stats = CacheStats::default();
    }
}

// Explanation for the presence of session_id that is currently unused: The intent was initially to
// have a conversation-unique identifier of some sort in order to isolate state between
// conversations. However, MCP provides no mechanism to distinguish between conversations, so I
// tried adding a session_id that was provided to every tool call in order to isolate state. This
// presents a usability concern, so I've decided to just be extra careful about switching contexts
// until we have a better solution. I still hope to iterate towards isolated sessions, so the code
// is still written to support that.

/// Session data specific to semantic editing operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SemanticEditSessionData {
    /// Current working context path
    pub context_path: Option<PathBuf>,
    /// Currently staged operation
    pub staged_operation: Option<StagedOperation>,
}

/// Represents a staged operation that can be previewed and committed
#[derive(Debug, Clone, Fieldwork, Serialize, Deserialize)]
#[fieldwork(get, set, get_mut, with)]
pub struct StagedOperation {
    pub selector: Selector,
    pub content: String,
    pub file_path: PathBuf,
    pub language_name: LanguageName,
    pub edit_position: Option<EditPosition>,
}

impl StagedOperation {
    pub fn retarget(&mut self, selector: Selector) {
        self.selector = selector;
    }
}

/// Semantic editing tools with session support
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
pub struct SemanticEditTools {
    #[fieldwork(get_mut)]
    session_store: SessionStore<SemanticEditSessionData>,
    language_registry: Arc<LanguageRegistry>,
    file_cache: Arc<Mutex<StatsLruCache>>,
    #[fieldwork(get)]
    file_operations: Box<dyn FileOperations>,
    #[fieldwork(set, with)]
    default_session_id: &'static str,
}

impl std::fmt::Debug for SemanticEditTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SemanticEditTools")
            .field("session_store", &self.session_store)
            .field("language_registry", &self.language_registry)
            .field("file_cache", &self.file_cache)
            .field("file_operations", &"<dyn FileOperations>")
            .field("default_session_id", &self.default_session_id)
            .finish()
    }
}

impl SemanticEditTools {
    /// Create a new SemanticEditTools instance with standard file operations
    pub fn new(
        storage_path: Option<&str>,
        file_operations: Box<dyn FileOperations>,
        cache_size: Option<NonZeroUsize>,
    ) -> Result<Self> {
        let storage_path = storage_path.map(|s| PathBuf::from(&*shellexpand::tilde(s)));
        let session_store = SessionStore::new(storage_path)?;
        let language_registry = Arc::new(LanguageRegistry::new()?);
        let cache_size =
            cache_size.unwrap_or_else(|| NonZeroUsize::new(50).expect("50 is non-zero"));
        let file_cache = Arc::new(Mutex::new(StatsLruCache::new(cache_size)));

        Ok(Self {
            session_store,
            language_registry,
            file_cache,
            file_operations,
            default_session_id: "default",
        })
    }

    /// Create a new SemanticEditTools instance with custom file operations
    /// Backward compatibility method that uses default cache size
    pub fn with_file_operations(
        storage_path: Option<&str>,
        file_operations: Box<dyn FileOperations>,
    ) -> Result<Self> {
        Self::new(storage_path, file_operations, None)
    }

    /// Create a new SemanticEditTools instance with standard file operations
    /// Convenience method that uses StdFileOperations and default cache size
    pub fn with_standard_operations(storage_path: Option<&str>) -> Result<Self> {
        Self::new(storage_path, Box::new(StdFileOperations), None)
    }

    /// Get context for a session
    pub fn get_context(&self, session_id: Option<&str>) -> Result<Option<PathBuf>> {
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());
        let session_data = self.session_store.get_or_create(session_id)?;
        Ok(session_data.context_path)
    }

    /// Stage a new operation, replacing any existing staged operation
    pub fn stage_operation(
        &self,
        session_id: Option<&str>,
        staged_operation: Option<StagedOperation>,
    ) -> Result<()> {
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());
        self.session_store.update(session_id, |data| {
            data.staged_operation = staged_operation;
        })
    }

    /// Get the currently staged operation, if any
    pub fn get_staged_operation(
        &self,
        session_id: Option<&str>,
    ) -> Result<Option<StagedOperation>> {
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());
        let session_data = self.session_store.get_or_create(session_id)?;
        Ok(session_data.staged_operation)
    }

    /// Take the staged operation, removing it from storage
    pub fn take_staged_operation(
        &self,
        session_id: Option<&str>,
    ) -> Result<Option<StagedOperation>> {
        let mut staged_op = None;
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());
        self.session_store.update(session_id, |data| {
            staged_op = data.staged_operation.take();
        })?;
        Ok(staged_op)
    }

    /// Modify the staged operation in place
    pub fn modify_staged_operation<F>(
        &self,
        session_id: Option<&str>,
        fun: F,
    ) -> Result<Option<StagedOperation>>
    where
        F: FnOnce(&mut StagedOperation),
    {
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());
        self.session_store.update(session_id, |data| {
            if let Some(ref mut op) = data.staged_operation {
                fun(op);
            }
        })?;
        self.get_staged_operation(Some(session_id))
    }

    /// Set context path for a session
    pub fn set_context(&self, session_id: Option<&str>, path: PathBuf) -> Result<()> {
        let session_id = session_id.unwrap_or_else(|| self.default_session_id());

        self.session_store.update(session_id, |data| {
            data.context_path = Some(path);
        })
    }

    /// Resolve a path relative to session context if needed
    pub(crate) fn resolve_path(&self, path_str: &str, session_id: Option<&str>) -> Result<PathBuf> {
        let path = PathBuf::from(&*shellexpand::tilde(path_str));

        if path.is_absolute() {
            return Ok(std::fs::canonicalize(path)?);
        }

        let session_id = session_id.unwrap_or_else(|| self.default_session_id());

        match self.get_context(Some(session_id))? {
            Some(context) => Ok(std::fs::canonicalize(context.join(path_str))?),
            None => Err(anyhow::Error::from(SemanticEditError::ContextNotFound {
                session_id: session_id.to_string(),
            })),
        }
    }

    /// Get file cache performance statistics
    pub fn cache_info(&self) -> Result<CacheStats> {
        let cache = self
            .file_cache
            .lock()
            .map_err(|_| SemanticEditError::CacheMutexPoisoned)?;
        Ok(cache.stats().clone())
    }

    /// Clear cache performance statistics
    pub fn clear_cache_stats(&self) -> Result<()> {
        let mut cache = self
            .file_cache
            .lock()
            .map_err(|_| SemanticEditError::CacheMutexPoisoned)?;
        cache.clear_stats();
        Ok(())
    }
}
