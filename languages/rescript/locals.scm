(switch_expression) @scope

(switch_match) @scope

(for_expression) @scope

(block) @scope

(function) @scope

; Definitions
;------------
(type_declaration) @definition.type

(let_binding) @definition.var

(module_declaration) @definition.namespace

(parameter
  (value_identifier) @definition.var)

(parameter
  (labeled_parameter
    (value_identifier) @definition.var))

(function
  parameter: (value_identifier) @definition.var)

; Safe destructuring bindings. Record patterns stay out here because the
; grammar flattens field names and bound names into sibling identifiers.
(tuple_item_pattern
  (value_identifier) @definition.var)

(array_pattern
  (value_identifier) @definition.var)

(list_pattern
  (value_identifier) @definition.var)

(dict_pattern_entry
  (value_identifier) @definition.var)

(spread_pattern
  (value_identifier) @definition.var)

(as_aliasing
  (value_identifier) @definition.var)

(formal_parameters
  (value_identifier) @definition.var)

(switch_match
  pattern: (value_identifier) @definition.var)

(for_expression
  (value_identifier) @definition.var)
