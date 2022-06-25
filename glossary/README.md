{% include "docs/glossary/_common/_glossary_header.md" %}

<!--The Fuchsia glossary uses templates to load definitions from _glossary.yaml. View
the fully rendered glossary at https://fuchsia.dev/fuchsia-src/glossary-->

# Glossary

The purpose of this document is to provide short definitions for a collection of
technical terms used in Fuchsia.

To add a glossary definition, see [Adding glossary entries](contribute/docs/glossary-entries.md).

## Terms

<div class="form-checkbox">
  <h4 class="showalways">Glossary area</h4>
<form id="filter-checkboxes-reset">
  {%- for area in areas %}
    {%- set found=false %}
    {%- for item in glossary -%}
        {%- for terma in item.area -%}
          {%- if terma == area %}
            {%- set found=true %}
          {%- endif %}
        {%- endfor %}
    {%- endfor %}
    {%- if found %}
      <div class="checkbox-div">
        <input type="checkbox" id="checkbox-reset-{{ area|lower|replace(' ','-')|replace('.','-') }}">
        <label for="checkbox-reset-{{ area|lower|replace(' ','-')|replace('.','-') }}">{{ area }}</label>
      </div>
    {%- endif %}
  {%- endfor %}
  <br>
  <br>
  <button class="select-all">Select all</button>
  <button class="clear-all">Clear all</button>
  <hr>
</form>

{% include "docs/glossary/_common/_glossary_list_header.md" %}
{%- for item in glossary %}
  {% include "docs/glossary/_common/_glossary_list_body.md" %}
{%- endfor %}
{% include "docs/glossary/_common/_glossary_list_footer.md" %}
{# This div is used to close the filter that is initialized above #}
</div>
