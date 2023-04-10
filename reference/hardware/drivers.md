{% include "docs/reference/hardware/_common/_driver_header.md" %}

# Fuchsia hardware drivers

{% comment %}
The list of Fuchsia drivers is generated from the information in the following
files:
https://fuchsia.dev/reference/drivers/all_drivers_doc.yaml

Since this page is generated from on a template, the full page is best viewed at
http://www.fuchsia.dev/fuchsia-src/reference/hardware/drivers
{% endcomment %}

This page lists drivers that are in the Fuchsia source tree. For deprecated
drivers, see [Deprecated drivers](/docs/reference/hardware/driver-epitaphs.md).

<a name="drivers"><h2>Drivers</h2></a>
<div class="form-checkbox">
  <h4 class="showalways">Driver area</h4>
<form id="filter-checkboxes-reset">
  {%- for area in drivers.drivers_areas %}
    {%- set found=false %}
    {%- for driver in drivers.drivers_documentation %}
        {%- for drivera in driver.areas %}
          {%- if drivera == area %}
            {%- set found=true %}
          {%- endif %}
        {%- endfor %}
    {%- endfor %}
    {%- if found %}
      <div class="checkbox-div">
        <input type="checkbox" value="area-{{ area|lower|replace(' ','-')|replace('.','-') }}"
        id="checkbox-reset-{{ area|lower|replace(' ','-')|replace('.','-') }}">
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
  <devsite-filter match="all" checkbox-form-id="filter-checkboxes-reset" sortable="0">
  <input type="text" placeholder="Find a driver" column="all">
{% include "docs/reference/hardware/_common/_index_table_header.md" %}
{%- for driver in drivers.drivers_documentation | sort(attribute='short_description') %}
        {% include "docs/reference/hardware/_common/_index_table_body.md" %}
{%- endfor %}
{% include "docs/reference/hardware/_common/_index_table_footer.md" %}
</div>