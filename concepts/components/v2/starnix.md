# Starnix

`starnix` is a [runner][glossary.runner] that allows for running
unmodified Linux binaries on Fuchsia.

Linux binaries are not run inside a virtual machine. Instead,
`starnix` creates a compatible runtime environment by implementing
the Linux UAPI.

Check out [`RFC-0082`][starnix-rfc] to learn more about the
motivation and design of `starnix`.

## Using starnix

The main entry point to `starnix` is the `ffx starnix` tool.

Check out the [`README`][starnix-readme] for instructions
to run Linux binaries, tests, or an interactive Android shell.

## Contributing

Patches that improve Linux compatibility are welcome. If you want
to report a bug or file a feature request, create an issue in the
[Starnix][starnix-monorail-component] component in Monorail.

[glossary.runner]: /glossary/README.md#runner
[starnix-rfc]: /contribute/governance/rfcs/0082_starnix.md
[starnix-readme]: /src/starnix/kernel/README.md
[starnix-monorail-component]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=component:Starnix
