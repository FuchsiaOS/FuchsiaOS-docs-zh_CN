# Layer repository structure

The garnet, peridot, and topaz layers share some structure.
This section documents these common aspects.

## public/

The `public/` directory defines the public interface to this repository. Code
outside this repository should not depend on files outside of this directory.
This directory should not depend on any files in this repository outside of this
directory. This property ensures that code outside this repository does not
transitively depend on any of the private files in this repository.

### public/build/

The `public/build/` directory contains files that support the build systems of
clients of the `public/` directory.

### public/fidl

The `public/fidl` directory contains the FIDL definitions for this layer,
which are language-agnostic definitions of interprocess communication
protocols. Each subdirectory corresponds to a FIDL library and has a name that
matches the name of the FIDL library.

### public/lib/

The `public/lib/` directory contains libraries (both static and dynamic) in
various languages that clients can link into their processes. Both the headers
and source files for these libraries are included in this directory.

Libraries that are private implementation details of this repository (i.e., not
part of this repository's public interface) should be in `lib/` instead.

## bin/

The `bin/` directory contains executable binaries. Typically, these binaries
implement one or more of the interfaces defined in `public/`, but the binaries
themselves are not part of the public interface of this repository.

## docs/

The `docs/` directory contains documentation about this repository.

## examples/

The `examples/` directory contains code that demonstrates how to use the public
interfaces exposed by this repository. This code should not be required for the
system to execute correctly and should not depend on the private code in this
repository.

## infra/

If the repository needs infrastructure related files, e.g., CQ configs,
then these files go here.

## lib/

The `lib/` directory contains libraries (both static and dynamic) that are used
internally by this repository. These libraries are internal implementation
details of this repository and should not be used by code outside this
repository.

Libraries that are part of the public interface of this repository (i.e., not
private implementation details) should be in `public/lib/` instead.

## manifest/

The `manifest/` directory contains jiri manifests for repo.

## packages/

The `packages/` directory contains package definitions for each
package in the repo.
