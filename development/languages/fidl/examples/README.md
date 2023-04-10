# FIDL examples

This is a catalog of FIDL examples intended to demonstrate FIDL concepts through
simplified implementations of real software workflows.

## Example index

The following examples sequentially demonstrate useful FIDL concepts.

<!-- DO_NOT_REMOVE_COMMENT:examples (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->

### Calculator

The [calculator][example_calculator] example shows fundamental building blocks
for creating your first FIDL protocol.

### Key-value store

The [key-value store][example_key_value_store] example demonstrates how to build
a simple key-value store using FIDL in order to learn about the various data
types available in the language.

### Canvas

The [canvas][example_canvas] example demonstrates how to build a simple 2D
line-rendering canvas using FIDL in order to learn about commonly used data flow
patterns.

<!-- /DO_NOT_REMOVE_COMMENT:examples (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->

## Concept index

Each "concept" in the FIDL language is exemplified in at least one of the
examples listed in the preceding section. A quick reference of each such
concept, as well as its example implementations, is listed in the following
section.

<!-- DO_NOT_REMOVE_COMMENT:concepts (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->

### Acknowledgement pattern

<<../widgets/_acknowledgement_pattern.md>>

### Alias

<<../widgets/_alias.md>>

### Anonymous type

<<../widgets/_anonymous_type.md>>

### Bits

<<../widgets/_bits.md>>

### Discoverable

<<../widgets/_discoverable.md>>

### Enum

<<../widgets/_enum.md>>

### Feed forward pattern

<<../widgets/_feed_forward_pattern.md>>

### Generated name

<<../widgets/_generated_name.md>>

### Handle rights

<<../widgets/_handle_rights.md>>

### Infallible two way method

<<../widgets/_infallible_two_way_method.md>>

### Named payload

<<../widgets/_named_payload.md>>

### Optionality

<<../widgets/_optionality.md>>

### Pagination pattern

<<../widgets/_pagination_pattern.md>>

### Persistence

<<../widgets/_persistence.md>>

### Protocol end

<<../widgets/_protocol_end.md>>

### Protocol

<<../widgets/_protocol.md>>

### Recursive type

<<../widgets/_recursive_type.md>>

### Resource type

<<../widgets/_resource_type.md>>

### Scalar type

<<../widgets/_scalar_type.md>>

### Size constraint

<<../widgets/_size_constraint.md>>

### Struct payload

<<../widgets/_struct_payload.md>>

### Table payload

<<../widgets/_table_payload.md>>

### Throttled event pattern

<<../widgets/_throttled_event_pattern.md>>

### Union payload

<<../widgets/_union_payload.md>>

<!-- /DO_NOT_REMOVE_COMMENT:concepts (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->

[example_calculator]: calculator/README.md
[example_canvas]: canvas/README.md
[example_key_value_store]: key_value_store/README.md
