# Key-value store: Improving the design {#variants}

Each of the following sections explores one potential way to iterate
on the original key-value store design, specifically:

+   [Adding support for reading from the store](#add_read_item)
+   [Using generic values](#use_generic_values)
+   [Supporting nested key-value stores](#support_trees)
+   [Adding support for iterating the store](#add_iterator)
+   [Enabling exporting backups](#support_exports)

Rather than building on one another sequentially, each
presents an independent way in which the base case presented
in the base case may be modified or improved.

This page builds on the
[Key-value store baseline example](/docs/development/languages/fidl/examples/key_value_store/README.md).

<!-- DO_NOT_REMOVE_COMMENT (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->

## Adding support for reading from the store {#add_read_item}

<<_add_read_item_tutorial.md>>

## Using generic values {#use_generic_values}

<<_use_generic_values_tutorial.md>>

## Supporting nested key-value stores {#support_trees}

<<_support_trees_tutorial.md>>

## Adding support for iterating the store {#add_iterator}

<<_add_iterator_tutorial.md>>

## Enabling exporting backups {#support_exports}

<<_support_exports_tutorial.md>>

<!-- /DO_NOT_REMOVE_COMMENT (Why? See: /tools/fidl/scripts/canonical_example/README.md) -->
