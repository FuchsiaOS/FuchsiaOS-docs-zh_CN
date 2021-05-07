# Fuchsia package URLs

A Fuchsia package URL looks like the following:

```
fuchsia-pkg://{{ '<var>' }}repository{{ '</var>' }}/{{ '<var>' }}package-name{{ '</var>' }}?hash={{ '<var>' }}package-hash{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

It consists of the following components, which form the full package URL:

* [URL scheme](#url-scheme): Required
* [Repository](#repository): Required
* [Package name](#package-name): Required
* [Package hash](#package-hash): Optional
* [Resource path](#resource-path): Optional

## URL scheme {#url-scheme}

The `fuchsia-pkg` URL scheme combines identifying characteristics to
establish a means for referring to a repository, a package, or a
resource, depending on which parts are included.

### Syntax

**Required**

```
{{ '<strong>' }}fuchsia-pkg://{{ '</strong>' }}<repository>[/<package-name>[?hash=<package-hash>][#<resource-path>]]
```

The scheme of a Fuchsia package are the case-insensitive characters, `fuchsia-pkg://`.

Although the canonical form is lower-case, URL scheme encoding is case-insensitive therefore
the system handles all cases.

## Repository {#repository}

The package URL contains a repository [hostname] to identify the package's
source. [RFC 1123] and [RFC 5890] specified that a hostname is a sequence of dot
(`.`)-delimited [IDNA A-labels], each of which consists of 1 to 63 of the
following latin-1 characters in any order:

* Digits (`0` to `9`)
* Lower-case letters (`a` to `z`)
* Hyphen (`-`)

No other characters are permitted.

The total maximum length of a hostname is 253 characters including the dots.

### Repository root verification (known sources)

The repository's root role (a quorum of one or more public or private key pairs)
establishes a chain of trust such that package authenticity, integrity, and
freshness can be verified cryptographically. The root role signs keys for more
limited roles which are then used to sign package metadata and the targets
themselves. See [TUF Security][TUF Security] and
[TUF roles and metadata][TUF METADATA] for more details.

To verify that a package is authentic, you must verify that the repository
from which it is being downloaded is authentic.

This will be implemented by maintaining a list of known source repositories
with their public keys on the device. Packages from unknown sources will
be rejected, although, on certain build types, new repositories can be added
at runtime.

### Syntax

**Required**

```
fuchsia-pkg://{{ '<strong>' }}<repository>{{ '</strong>' }}/<package-name>?hash=<package-hash>#<resource-path>
```

#### Examples

`fuchsia-pkg://{{ '<strong>' }}fuchsia.com{{ '</strong>' }}`

## Package name {#package-name}

A package name is a symbolic label that identifies a logical collection of
software artifacts (files), independent of any particular variant or revision
of those artifacts. The package name is used to locate package metadata within
a repository. Package metadata must be signed by a role which is trusted by
the repository root.

A package name consists of a sequence of up to 100 of the following latin-1
characters in any order:

* Digits (`0` to `9`)
* Lower-case letters (`a` to `z`)
* Hyphen (`-`)
* Underscore (`_`)
* Period (`.`)

No other characters are permitted.

Each package name must be unique among all packages in a repository.
Packages within different repositories are considered distinct even
if they have the same name.

### Syntax

**Required**

```
fuchsia-pkg://<repository>/{{ '<strong>' }}<package-name>{{ '</strong>' }}?hash=<package-hash>#<resource-path>
```

There must be a single `/` character between the repository and [package name](#package-name).

#### Examples

* `fuchsia-pkg://fuchsia.com/{{ '<strong>' }}fuchsia-shell-utils{{ '</strong>' }}`
* `fuchsia-pkg://fuchsia.com/{{ '<strong>' }}fuchsia-shell-fonts{{ '</strong>' }}`
* `fuchsia-pkg://fuchsia.com/{{ '<strong>' }}fuchsia-shell-scenic{{ '</strong>' }}`

## Package hash {#package-hash}

A package hash is the [merkleroot] of the package's meta.far.  Because the
package's metadata encodes the content addresses of the package's files, any
changes to the package's metadata or content will produce a different package
hash, thereby making it possible to distinguish each unique revision of the
package.

If the package hash is missing, the package resolver fetches the resources
from the newest revision of the package available to the client.

A package hash is represented as a hex-encoded string consisting of exactly 64
of the following latin-1 characters: digits (`0` to `9`) and lower-case letters
(`a` to `f`).  No other characters are permitted.

### Syntax

**Optional**

```
fuchsia-pkg://<repository>/<package-name>{{ '<strong>' }}?hash=<package-hash>{{ '</strong>' }}#<resource-path>
```

Only valid if a package name is specified.

Must begin with the string `?hash=` followed by the [package hash](#package-hash).

#### Examples

`fuchsia-pkg://google.com/chrome/stable{{ '<strong>' }}?hash=80e8721f4eba5437c8b6e1604f6ee384f42aed2b6dfbfd0b616a864839cd7b4a#meta/webview.component{{ '</strong>' }}`

## Resource path {#resource-path}

A resource path is a UTF-8 string that identifies a resource within a package.
This is a file path, consisting of a sequence of single `/` delimited
path segments, each of which is a non-empty sequence of non-zero UTF-8
characters not equal to `.`, `..`, or `/`. Must begin with single `#` character.

This must be relative to the root of the package.

Note: The scheme, [repository hostname](#repository-hostname),
[package name](#package-name), [package variant](#package-variant), and [package
hash](#package-hash) components are all defined to use a restricted subset of
characters, none of which require encoding, unlike the resource path.

URL components containing reserved characters are percent-encoded according to
[RFC 3986]. This definition is compatible with the definition of [Fuchsia filesystem paths]
but it imposes a UTF-8 encoding rather than admitting arbitrary binary strings
since such strings cannot always be encoded as valid URLs.

For example, `hello/unicode/%F0%9F%98%81` decodes to `hello/unicode/😁`.

### Syntax

**Optional**

Only valid if a package was specified.

```
fuchsia-pkg://<repository>/<package-name>?hash=<package-hash>{{ '<strong>' }}#<resource-path>{{ '</strong>' }}
```

#### Examples

* `fuchsia-pkg://fuchsia.com/fuchsia-shell-utils/stable{{ '<strong>' }}#bin/ls{{ '</strong>' }}`
* `fuchsia-pkg://google.com/chrome/stable{{ '<strong>' }}#meta/webview.component{{ '</strong>' }}`
* `fuchsia-pkg://google.com/chrome/stable{{ '<strong>' }}#lib/mylibrary.so{{ '</strong>' }}`

<!--xrefs-->
[TUF Specification]: https://github.com/theupdateframework/specification/blob/HEAD/tuf-spec.md#4-document-formats
[TUF Security]: https://theupdateframework.github.io/security.html
[TUF Metadata]: https://theupdateframework.github.io/metadata.html
[hostname]: https://en.wikipedia.org/wiki/Hostname
[RFC 1123]: https://tools.ietf.org/html/rfc1123
[RFC 5890]: https://tools.ietf.org/html/rfc5890
[IDNA A-labels]: https://tools.ietf.org/html/rfc5890#section-2.3.2.1
[Fuchsia filesystem paths]: /docs/concepts/process/namespaces.md#object-relative-path-expressions
[RFC 3986]: https://tools.ietf.org/html/rfc3986#page-11
[merkleroot]: /docs/concepts/packages/merkleroot.md
