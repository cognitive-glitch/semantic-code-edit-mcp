;; Tree-sitter validation queries for TypeScript semantic editing
;; Minimal working validation

;; CRITICAL: Return statements outside of functions
((return_statement) @invalid.return.outside.function
 (#not-has-ancestor? function_declaration))