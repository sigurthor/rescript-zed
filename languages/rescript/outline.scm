; Let declarations, including `let rec`
(let_declaration
  [
    "let"
    "export"
  ] @context
  (let_binding
    pattern: (_) @name) @item)

; Type declarations, including `type rec` and `export type`
(type_declaration
  "type" @context
  (type_binding
    name: (_) @name) @item)

; Module declarations, including `module rec` and `module type`
(module_declaration
  "module" @context
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

; Record type fields
(record_type_field
  (property_identifier) @name) @item

; Object type fields
(object_type
  (field
    (property_identifier) @name) @item)
