# The CTF Test Collection

[CTF] tests run in a special collection which is meant to isolate any capabilities
that are required only for CTF testing. CTF tests must declare the following facet
so they run in this collection:

```json5
// my_test.cml

{
    include: [
        {{ '<strong>' }}"//sdk/ctf/test_realm/meta/test-collection.shard.cml",{{ '</strong>' }}
        "//src/sys/test_runners/rust/default.shard.cml",
    ],
    program: {
        binary: "bin/my_test_binary",
    },
}
```

The system capabilities offered to CTF tests can be found in
//sdk/ctf/test_realm/meta/test-collection.shard.cml.

[CTF]: /docs/development/testing/ctf/overview.md
