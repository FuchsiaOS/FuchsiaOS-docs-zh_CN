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
