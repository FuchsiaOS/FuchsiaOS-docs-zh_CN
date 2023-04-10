{% set areas | yamlloads %}
{% include "docs/contribute/governance/areas/_areas.yaml" %}
{% endset %}

# Areas

{% for area in areas %}
## {{ area.name }} {:#{{ area.name|replace(" ", "-")|lower() }}}

**Primary**: {{ area.api_primary }} \
{% if area.api_secondary %}
**Secondary**: {{ area.api_secondary }}
{% endif %}

{{ area.description or "No description available." }}

{% if area.examples %}
Examples:

<ul>
  {% for example in area.examples %}
  <li>
    {% if example.fidl %}
    <a href="https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/{{example.fidl}}">{{example.fidl}}</a>
    {% endif %}
    {{ example.description or ""}}
  </li>
  {% endfor %} <!-- for example in area.examples -->
</ul>
{% endif %} <!-- if area.examples -->

{% endfor %} <!-- for area in areas -->
