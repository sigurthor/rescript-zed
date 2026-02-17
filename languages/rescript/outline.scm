; Let declarations, e.g. `let foo = 42`
(let_declaration
["let" "export"] @context
(let_binding
pattern: (_) @name) @item)

; Recursive let declarations, e.g. `let rec fib = n => ...`
(let_declaration
["let" "export"] @context
"rec" @context
(let_binding
pattern: (_) @name) @item)

; Type declarations, e.g. `type t = int`
(type_declaration
"type" @context
(type_binding
name: (_) @name) @item)

; Recursive type declarations, e.g. `type rec tree<'a> = ...`
(type_declaration
"type" @context
"rec" @context
(type_binding
name: (_) @name) @item)

; Exported type declarations, e.g. `export type t = int`
(type_declaration
"export" @context
"type" @context
(type_binding
name: (_) @name) @item)

; Module declarations, e.g. `module Foo = { ... }`
(module_declaration
"module" @context
(module_binding
name: (_) @name) @item)

; Recursive module declarations
(module_declaration
"module" @context
"rec" @context
(module_binding
name: (_) @name) @item)

; Module type declarations
(module_declaration
"module" @context
"type" @context
(module_binding
name: (_) @name) @item)

; External declarations
(external_declaration
"external" @context
(value_identifier) @name) @item

; Exception declarations
(exception_declaration
"exception" @context
(variant_identifier) @name) @item

; Open statements
(open_statement
"open" @context
(_) @name) @item

; Include statements
(include_statement
"include" @context
(_) @name) @item

; Variant declarations inside type bodies
(variant_declaration
(variant_identifier) @name) @item
