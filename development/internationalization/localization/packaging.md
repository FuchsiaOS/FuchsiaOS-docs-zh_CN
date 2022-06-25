# Packaging

The generated JSON file from the previous section must be bundled together with
the Fuchsia program so that it can be found at program runtime.
See: [Provide data files to components][provide-data-guide].

We have established some conventions for packaging resources (i.e. localized
assets). The schema is intended to be extensible to other asset types, and also
to be able to support _combinations_ of asset types, which are sometimes useful
to have when expressing more complex relationships between device and locale (a
Hebrew icon version for a 200dpi display).  All paths below are relative to the
package's data directory and are found under `/pkg/data` on a running system.

| **Path** | **Description** |
|----------|-----------------|
| `assets/` | Stores all assets.  This is similar to how the <code>meta/</code> directory contains package manifests and other metadata.  In the future, this directory could contain conventional indices. |
| `assets/locales` | Stores data specifically for locales |
| `assets/locales/fr-fr` | Stores data for particular locales.  The locale names are individual directories in [BCP47](https://tools.ietf.org/html/bcp47) format. Each program contributes a single JSON file to this directory, named `program.json`, where the `program` part of the name is chosen by the author. We will, at some point, probably need to ensure that package and library names for files here do not collide. Also, due to Fuchsia's packaging strategy, it may pay to have many smaller files storing translations instead of one large one, for ease of update. |

The integrity of the path structure and proper packaging of localized resources
still needs to be done manually.

## Packaging localized resources

Packaging localized resources is best illustrated with an example.  Let us start
with the `BUILD.gn` file and we can focus on particular sections later.

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/intl/example/BUILD.gn" adjust_indentation="auto" %}
```

### Build rule to generate the localized resources

Localized resources are based on the files on the filesystem.  An example
program at [//src/lib/intl/example](/src/intl/example/README.md) shows how
you can build and deploy a Fuchsia program that contains localized messages.

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/intl/example/BUILD.gn" region_tag="strings" adjust_indentation="auto" %}
```

The build rule `intl_strings` instructs the build system to process the XML
files containing strings.  Refer to the section on [message
translation](message_translation.md) to learn more about the translation
workflow.  The translated messages are available in the default build output
directory for this rule.  The line `source = "strings.xml"` points to the file
containing the message sources.  Since these messages are always written using
a spoken language, and since no particular language is more relevant than
others, you also need to let the system know the language that the default
`strings.xml` is written in.

To set the default language to English, in your `BUILD.gn` add `source_locale =
"en"`.  The declaration of output locales allow you to specify the exact list
of locale resources that you want generated.  This is what `output_locales` is
used for.  This explicit declaration is for several reasons:

* You want to have an explicitly declared list of locales you require for your
  program to be available at runtime.

* The build system requires that the input and output files be declared
  explicitly for build correctness reasons.  Since the input and output file
  names are generated based on the locales, the locale declaration is enough
  to handle this issue.

For example, if the original file name is `strings.xml` and the locale `fr` is
listed in the `output_locales`, the build system expects a file named
`strings_fr.xml`.  These resources will need to be packaged with the binary
into a Fuchsia package.

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/intl/example/BUILD.gn" region_tag="resources" adjust_indentation="auto" %}
```

Make sure to package the JSON resource into the correct directory.  For
translated messages, the correct directory path would be
`assets/locales/fr/l10n.json` for the translated messages for French.  It is
also required to package the ICU data, is defined in the `icudtl.dat` section.

[provide-data-guide]: development/components/data.md
