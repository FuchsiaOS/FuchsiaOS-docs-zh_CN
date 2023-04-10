# Go

- [Logging](logging.md)
- [Readability reviews](readability_reviews.md)

## Development setup

The layout of the Fuchsia checkout is not compatible out-of-the-box with
standard Go tooling like `go test` and `gopls` (the Go language server).

To set up your workspace for compatibility with Go tooling, run `fx setup-go`.
This will create all of the symlinks and other files necessary for Go tooling
and to work and be compatible with IDEs.

Note that this is only necessary if you care about IDE features and/or running
`go` commands directly. If you're happy without IDE features and with using `fx
set`, `fx build`, and `fx test` to work with Go, feel free to skip this setup.

### Editor configuration

Your editor needs to be configured to properly use Fuchsia-vendored Go build
tooling. In particular:
- The GOROOT needs to be set to the Fuchsia-aware GOROOT (likely to be
  `$FUCHSIA_DIR/out/default/host-tools/goroot`, though make sure to substitute
  `$FUCHSIA_DIR` for the actual path and `out/default` for the GN out directory
  you are using).
- "fuchsia" needs to be added to the go build tags.
