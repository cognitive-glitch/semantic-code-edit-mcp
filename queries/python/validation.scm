;; Tree-sitter validation queries for Python semantic editing
;; Minimal working validation

;; CRITICAL: Return statements outside of functions
((return_statement) @invalid.return.outside.function
 (#not-has-ancestor? function_definition))