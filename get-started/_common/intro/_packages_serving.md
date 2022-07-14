## Hosting and serving packages

Packages are hosted in **repositories** based on
[The Update Framework](https://theupdateframework.io/){:.external} (TUF).
This framework is a specification designed to enable secure delivery of software
updates. TUF repositories secure updates through signed metadata attached to
records that are verifiable against known trusted public and private keys.
This means that **any HTTP server can serve a TUF repository** without the need
for transport-level security, **_including a developer's workstation!_**

<aside class="key-point">
Developer tools such as <code>ffx</code> host a simple HTTP server locally
serving a static tree of files formatted as a TUF repository.
</aside>

Packages within a repository are identified by a URL with the
`fuchsia-pkg` scheme:

```none
fuchsia-pkg://{{ '<var>' }}repo-hostname{{ '</var>' }}/{{ '<var>' }}pkg-name{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

* `repo-hostname`: Hostname of a trusted package repository, such as `fuchsia.com`.
* `pkg-name`: Unique identifier for the package in this repository.
* `resource-path`: Resource contained within the package, such as a component
  manifest.

![Diagram showing how packages are resolved from a TUF repository and cached
locally on the device.]
(/get-started/images/intro/package-resolver.png){: width="751"}

Requests for software on a Fuchsia device are handled by the
**package resolver**. The package resolver determines if the system already has
the package cached locally. If not, the resolver fetches the meta.far from the
repository and updates the necessary content BLOBs.
