# Automated documentation checks

The command line tool `doc-checker` performs several checks on the documentation
source. Checks that do not need to access external links are performed as
presubmit checks when submitting changes to the //docs directory.

The primary goal of doc-checker is to make sure all the documents in the `//docs`
directory are part of the interconnected graph made up of `_toc.yaml` files and
in-page links are reachable when the documentation is published on
fuchsia.dev. Other checks inspect the links themselves to enforce documentation
standards and consistency.

## Running doc-checker

```sh
fx doc-checker
```

The external link check can be skipped by adding `--local-links-only`.

For more options, see the [full command line reference](https://fuchsia.dev/reference/tools/fx/cmd/doc-checker.md).

If it is not part of the current build configuration for some reason,
re-run `fx set` including the option to build doc_checker:
`--with //tools/doc_checker:doc_checker`.

These are the situations that are reported by doc-checker:

## Internal link checks

### Links to nonexistent files

```markdown
[missing doc](/does_not_exist.md)
```

### Links to files in //docs represented as full URLs

These are links that reference `fuchsia.googlesource.com` or `fuchsia.dev` and
access a file in //docs. These links should be converted to file paths.

#### Incorrect

```markdown
[unnessary link](https://fuchsia.dev/fuchsia-src/get-started/learn-fuchsia.md)
```

#### Correct

```markdown
[correct link](/get-started/learn-fuchsia.md)
```

### Links to inactive Fuchsia projects

These are projects that are included in the Fuchsia source tree which have been
merged into Fuchsia or completed deprecated. The list of [valid projects](https://source.corp.google.com/fuchsia/tools/doc_checker/src/link_checker.rs;l=49])
is part of the source code.

```markdown

     [invalid old project](https://fuchsia.googlesource.com/garnet/+/refs/heads/main)
```

### Links with relative paths past //docs

These are links that point to paths beyond the //docs directory. These should be
converted to `fuchsia` relative paths.

#### Incorrect

```markdown
  [source file](/docs/../src/BUILD.gn)
```

#### Correct

```markdown
   [source file](/src/BUILD.gn)
```

### Missing `alt` text for images

Images must have meaningful `alt` text.

```markdown
![Diagram of the state transitions](/state-machine.png "State machine")
```

### Including markdown fragment files

Markdown file fragments are included in another markdown file by using

<pre>
&lt;&lt;relative-path-to/_file.md&gt;&gt;
</pre>

The path must be relative to the current .md source file—absolute paths
cannot be used. The `<< >>` directive is a block directive and so must appear
on a line by itself.

## YAML data file checks

YAML files in fuchsia.dev are used to store documentation content in a structured
format which is then rendered through the use of Jinja templates. Any YAML file
that is prefixed with a `_` indicates that the YAML is not published as a
standalone file, but only rendered through the use of a template. If a YAML
file is not prefixed with `_`, the YAML file is published on fuchsia.dev and can
be viewed as a plain YAML file.

### _toc.yaml checks

`_toc.yaml` files are mainly used to create the information architecture for
fuchsia.dev.

These checks enforce the table of contents structure described in
 [_toc.yaml reference](/contribute/docs/documentation-navigation-toc.md#toc-reference).

* top level key `toc`
* entries are one of:
  * `break: true` - _(optional)_ Adds a vertical break
  * `contents: <list of toc entries>` - _(optional)_ The contents of a custom
   tab.
  * `heading: <string>` - _(optional)_ The heading for a group of links.
  * `include: <path to _toc.yaml>` - _(optional)_ Includes another _toc.yaml.
  * `name: <string>` - _(optional)_ Name of this tab.
  * `path: <string>` - _(optional)_ Path or URL to a page.
  * `path_attributes: <mapping>` - _(optional)_  Name-value pairs of attributes
   for the link created based on the `path` property.
  * `section: <toc entry>` - _(optional)_ Indented toc entry defining a
   collapsible section, usually defined via `include` another _toc.yaml file.
  * `skip_translation: true` - _(optional)_ Prevents human and machine
   translation of all link titles in this entry and any descendents.
  * `status: <string>` - _(optional)_  Used with `heading` or `title` and
   cannot be used with `break` or `include`. Applies a predefined status. The
   status must be one of:
    * `alpha`
    * `beta`
    * `deprecated`
    * `experimental`
    * `external`
    * `limited`
    * `new`
  * `step_group: <string>` -  _(optional)_  Used to create groups of content
    that have `prev` and `next` navigation links to the bottom of the pages.
  * `style: <string>` -  _(optional)_  Cannot be used with `break` or `include`.
    This style is applied to the `heading` or `section` element. This value must
    be `accordion`.
  * `title: string` - _(optional)_ The link title.

* `path` properties are valid paths:
  * File paths such as `/somewhere/file.md`
  * `http://` or `https://` URLs.
  * `/reference` to generated reference documentation. These links are validated
   using external links to `fuchsia.dev/reference`.
  * Special files: `/CONTRIBUTING.md` and `/CODE_OF_CONDUCT.md`.

### Pages not referenced in _toc.yaml graph

Markdown pages in //docs must appear in a `_toc.yaml` that is included in the
 graph of table of contents created from the root _toc.yaml in
 `//_toc.yaml`.

### Structure of _areas.yaml

TBD: _What is the structure of ____areas___.yaml__

### Structure of _eng_council.yaml

TBD: _What is the use of ___eng_council.yaml___?_

### Structure of _metadata.yaml

TBD: _What is the use of ___metadata.yaml___?_

### Structure of _rfcs.yaml

This file defines the metadata for RFC documents.

See [RFC metadata](/contribute/governance/rfcs/create_rfc.md#create-metadata).

### Structure of_roadmap.yaml

TBD: _What is the use of ___roadmap.yaml___?_

### Structure of_drivers_areas.yaml

TBD: _What is the use of ___drivers_areas.yaml___?_

### Structure of _drivers_epitaphs.yaml

TBD: _What is the use of ____drivers_epitaphs_.yaml___?_

### Structure of_problems.yaml

TBD: _What is the use of ___problems.yaml___?_

### Structure of_redirects.yaml

This file defines redirections to another page for a given URL.

### Structure of _supported_cpu_architecture.yaml

TBD: _What is the use of ___supported_cpu_architecture.yaml___?_

### Structure of_supported_sys_config.yaml

List of supported systems configurations.

See [Supported system config list](/contribute/governance/rfcs/0130_supported_hardware.md#initial_supported_sys_configyaml_list)

### Structure of_tools.yaml

TBD: _What is the use of ___tools.yaml___?_

See: [source code](https://source.corp.google.com/fuchsia/tools/doc_checker/src/yaml.rs;l=185)

### Structure of_deprecated-docs.yaml

This file defines the redirection rules for deprecated documents.

See: [Redirect the pages to the deprecation notice](/contribute/docs/deprecating-docs.md#redirect_the_pages_to_the_deprecation_notice).

### Structure of glossary.yaml

This file defines the glossary of Fuchsia terminology.

See: [Adding a glossary term](/contribute/docs/glossary-entries.md#add_a_glossary_entry)

## External link checks

Note: External links are not checked during presubmits. These checks can be
skipped by adding the `--local-links-only` flag.

### Broken external links (resulting in 404)

```markdown
    [broken link](https://mispeeled.com)
```

### Including the `hl` parameter for Google hosted sites

The `hl` parameter indicates the user's host language. This parameter should
 not be included in the URL since it disables the automatic redirection to
 translated pages if they exist.
