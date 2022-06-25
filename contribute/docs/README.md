{% import 'docs/_common/_doc_widgets.md' as widgets %}

# Contributing to documentation

Pages on Fuchsia.dev are written in markdown (.md) and use the
[Hoedown][hoedown-git]{: .external} markdown flavor.

Fuchsia.dev also supports [Jinja2 templating][jinja].

The following pages list reference material for creating documentation for
Fuchsia.dev:

{% set tocmeta | yamlloads %}
{% include "docs/contribute/docs/_toc.yaml" %}
{% endset %}
{{ widgets.inline_toc () }}

<!-- xrefs -->

[hoedown-git]: https://github.com/hoedown/hoedown
[jinja]: https://jinja.palletsprojects.com/en/2.11.x/

