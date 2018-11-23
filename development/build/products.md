# Products

**Products** are defined in JSON files which can be found at:

* Garnet Layer Products: [`//garnet/products/`][garnet-products-source].
* Peridot Layer Products: [`//peridot/products/`][peridot-products-source].
* Topaz Layer Products: [`//topaz/products/`][topaz-products-source].

Products are a Fuchsia-specific feature built on top of GN to help customize
Fuchsia builds. Products reference [packages](packages.md) and coarsely
define which build artifacts the packages are added to.

## Package Sets

A product can import one or more packages into three different package sets
of build artifacts, as defined below. The package sets influence what
packages are included in parts of build output.

### monolith

The `monolith` section of a **product** defines the list of [build
packages](packages.md) that are to be included in the disk images, system
update images and package repository. Membership of a package in the
`monolith` dependency set takes precedence over membership in other package
sets.

### preinstall

The `preinstall` section of a **product** defines the list of [build
packages](packages.md) that are to be preinstalled in the disk image
artifacts of the build, and will also be made available in the package
repository. These packages are not added to the system update images or
packages.

### available

The `available` section of a **product** defines the list of [build
packages](packages.md) that are added to the package repository only. These
packages will be available for runtime installation, but are not found in
system update images nor are they preinstalled in any disk images. All
members of `monolith` and `preinstall` are inherently `available`.

## Defaults & Conventions

### product: default

The `default` product for a layer, found in `//<layer>/products/default` by
convention contains:

* `monolith` - a common minimal base for this layer that makes up a system
  update.
* `preinstall` - a set of most commonly used development tools for the layer
  and other common work-items.
* `available` - all `prod` packages for the layer.

By convention, the `default` product for a higher layer should be additive
from the layer below.

## Inspecting Products

As products reference [packages](packages.md) and packages may reference
other packages, it is useful to be able to inspect the expanded and filtered
set of build labels that will make up each package set in a product. The
[preprocess products][preprocess-products-py] script is the tool that
produces this for the build and can be run by hand:

```bash
$ python build/gn/preprocess_products.py --products '["garnet/products/default"]'
```

[garnet-products-source]: https://fuchsia.googlesource.com/garnet/+/master/products/
[peridot-products-source]: https://fuchsia.googlesource.com/peridot/+/master/products/
[topaz-products-source]: https://fuchsia.googlesource.com/topaz/+/master/products/
[preprocess-products-py]: https://fuchsia.googlesource.com/build/+/master/gn/preprocess_products.py
