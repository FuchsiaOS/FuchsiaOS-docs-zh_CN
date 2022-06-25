{% include "docs/reference/hardware/_common/_driver_header.md" %}

# Deprecated Fuchsia hardware drivers

{% comment %}
The list of Fuchsia drivers is generated from the information in the following
files:
reference/hardware/_drivers_epitaphs.yaml

Since this page is generated from on a template, the full page is best viewed at
http://www.fuchsia.dev/fuchsia-src/reference/hardware/driver-epitaphs
{% endcomment %}

This page lists drivers that are deprecated in the Fuchsia source tree, For the
drivers in the Fuchsia source tree, see [Fuchsia hardware drivers](drivers.md).

<a name="deprecated-drivers"><h2>Deprecated drivers</h2></a>
  <div class="form-checkbox">
  <h4 class="showalways">Driver area</h4>
<form id="filter-checkboxes-reset-2">
  {%- for area in areas %}
    {%- set found=false %}
    {%- for epitaph in epitaphs %}
        {%- for drivera in epitaph.areas %}
          {%- if drivera == area %}
            {%- set found=true %}
          {%- endif %}
        {%- endfor %}
    {%- endfor %}
    {%- if found %}
      <div class="checkbox-div">
        <input type="checkbox" value="area-{{ area|replace(" ", "-") }}"
        id="checkbox-reset-deprecated-{{ area|replace(" ", "-") }}">
        <label for="checkbox-reset-deprecated-{{ area|replace(" ", "-") }}">{{ area }}</label>
      </div>
    {%- endif %}
  {%- endfor %}
  <br>
  <br>
  <button class="select-all">Select all</button>
  <button class="clear-all">Clear all</button>
  <hr>
</form>
    <devsite-filter match="all" checkbox-form-id="filter-checkboxes-reset-2" sortable="0">
  <input type="text" placeholder="Find a driver" column="all">
{% include "docs/reference/hardware/_common/_index_table_header.md" %}
{%- for epitaph in epitaphs | sort(attribute='short_description') %}
        {% include "docs/reference/hardware/_common/_index_table_body_deprecated.md" %}
{%- endfor %}
{% include "docs/reference/hardware/_common/_index_table_footer.md" %}
{# This div is used to close the filter that is initialized above #}
</div>