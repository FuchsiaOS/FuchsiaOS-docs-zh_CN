## Identifying components

Components are identified by a URL. The framework resolves component URLs to
component declarations with the help of a **component resolver**. Resolvers are
components themselves that are capable of handling a particular URL scheme and
fetching the component manifest, program, and assets.

Most components are published inside a Fuchsia package, so the component URL is
a reference to the component manifest inside that package. See the following
example:


```none
fuchsia-pkg://fuchsia.com/{{ '<var>' }}foo-package{{ '</var>' }}#meta/{{ '<var>' }}foo-component.cm{{ '</var>' }}
```

Component instances are identified by a topological path reference known as a
**moniker**. A component's moniker indicates its location within the component
instance tree as an absolute or relative path. For example, the moniker path
`/core/system-updater` refers to the instance of `system-updater` that exists
in the `core` realm.
