# `package()` migration

## Goal & motivation

In 2020 we introduced [new build templates][building-components] for defining
packages, components, and their tests. The new templates more closely reflect
the project's architectural goals and direction. They replace old templates from
2016, mainly `package()`, `test_package()`, and `unittest_package()`.

Some of the benefits of the new templates include:

*   Less repetition and a more succinct syntax. From experience with migrating
    old targets to the new templates, the new definitions span 30%-50% fewer
    lines.
*   Components can be defined once, and then used in multiple packages. For
    instance reuse the same component in the definition of the production
    package that participates in a product definition, and the package
    containing an integration test between your component and other components.
*   Since components and packages can be defined separately, the build system
    knows which dependencies belong in which component, and can therefore
    perform more sophisticated build-time validation for you, detecting errors
    automatically and providing clearer feedback.
*   More use cases are supported natively rather than requiring special syntax.
    For instance, packages that contain drivers, additional data files, or
    shared libraries don't require special syntax - just add `deps` on the build
    targets that define what you want your component to use or your package to
    contain.
*   It's easier to define simple tests that don't require manually writing a
    component manifest (`.cml` or `.cmx` file).
*   Better documentation is available, covering more languages and use cases.

Migrating to the new templates makes our build definitions more readable and
more useful, and helps clean underlying tech debt related to packaging that is
kept alive only in service of the legacy `package()` template. Please help us
replace all uses of the old templates with the new ones so we can move forward.

## Technical background

Please review the guide on [building components][building-components].

General experience in working with `BUILD.gn` files is recommended but not
entirely necessary.
Please consult the [GN reference][gn-reference]{:.external} guide.

## How to help

### Picking a task

The best place to start is the list in
[`//build/BUILD.gn`](/build/BUILD.gn)
under the group `"deprecated_package"`.

Find a directory that you're familiar with, or pick one at random.
In this directory's `BUILD.gn` file you will likely see build targets with the
old templates, for instance:

```gn
package("foo") {
  ...
}
```

Follow the [migration guide][migration-guide] or the examples below to convert
to the new templates.

### Doing a task

In your directory of choice, remove all uses in the `BUILD.gn` file of the
templates `package()`, `test_package()`, or `unittest_package()`. Note that
there may be other templates used in the file that wrap these templates. Make
sure you got them all by deleting the associated line in
[`//build/BUILD.gn`](/build/BUILD.gn)
and seeing that you're still able to generate the build (run `fx gen` and see
that GN successfully generated the Ninja file without errors).

### Completing a task

When preparing your change, make sure to remove any lines from
[`//build/BUILD.gn`](/build/BUILD.gn)
listing the directories that you cleaned up.

Send code reviews to owners of the directories with the build definitions that
you're changing, or to people listed below who volunteered to help with these
migrations:

*   shayba@google.com
*   xbhatnag@google.com

New volunteer? Please add yourself to the list!

## Examples

*   [464409: [time] Migrate all packages + tests to new build rules](https://fuchsia-review.googlesource.com/c/fuchsia/+/464409)
*   [464407: [build] Migrate tests in //src/sys/lib to new build rules](https://fuchsia-review.googlesource.com/c/fuchsia/+/464407)
*   [464410: [build] migrate packages in //src/sys to new build rules](https://fuchsia-review.googlesource.com/c/fuchsia/+/464410)
*   [473597: [session] Migrate all packages and tests to new build rules] (https://fuchsia-review.googlesource.com/c/fuchsia/+/473597)

## Sponsors

Reach out for questions or for status updates:

*   <shayba@google.com>
*   <xbhatnag@google.com>

[building-components]: /docs/development/components/build.md
[gn-reference]: https://gn.googlesource.com/gn/+/master/docs/reference.md
[migration-guide]: /docs/development/components/build.md#legacy-package-migration
