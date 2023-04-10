{% include "docs/contribute/roadmap/_common/_roadmap_header.md" %}

# Fuchsia 2022 roadmap overview

{% comment %}
The list of Fuchsia roadmap items for 2022 is generated from the information in
the following files:
/docs/contribute/roadmap/2022/_roadmap.yaml

Since this page is generated from on a template, the full page is best viewed at
http://www.fuchsia.dev/fuchsia-src/contribute/roadmap/2022
{% endcomment %}

{% dynamic if user.is_googler %}

{% include "contribute/roadmap/2022/_yaml_load.md" %}
{% include "docs/contribute/roadmap/_common/_roadmap_body_2022.md" %}

{% dynamic else %}

{% include "docs/contribute/roadmap/_common/_yaml_load.md" %}
{% include "docs/contribute/roadmap/_common/_roadmap_body_2022.md" %}

{% dynamic endif %}