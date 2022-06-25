{% include "docs/reference/hardware/_common/_sys_header.md" %}

# Supported system configurations

{% comment %}
The list of Fuchsia supported system configurations is generated from the
information in the following files:
reference/hardware/_supported_sys_config.yaml

Since this page is generated from a template, the full page is best viewed at
http://www.fuchsia.dev/fuchsia-src/reference/hardware/supported-system-config
{% endcomment %}

Note: This page was implemented based on
[RFC-0130](contribute/governance/rfcs/0130_supported_hardware.md).

This page displays supported system configurations for running Fuchsia.

<a name="system-config"><h2>Supported system configuration</h2></a>
<div class="form-checkbox">
  <h4 class="showalways">CPU architecture</h4>
  <aside class="note"><b>Note:</b> Fuchsia source uses x64 to refer to x86-64.</aside>
<devsite-select id="filter-selection">
  <select>
  <option>Select a CPU architecture</option>
  {%- for arc in architecture %}
    {%- set found=false %}
    {%- for sys in sysconfig %}
          {%- if arc == sys.architecture %}
            {%- set found=true %}
          {%- endif %}
    {%- endfor %}
    {%- if found %}
      <option>{{ arc }}</option>
    {%- endif %}
  {%- endfor %}
  </select>
</devsite-select>
  <devsite-filter match="all" select-el-container-id="filter-selection" sortable="0">
  <br>
  <input type="text" placeholder="Find a supported system configuration" column="all">
{% include "docs/reference/hardware/_common/_sys_index_table_header.md" %}
{%- for sys in sysconfig | sort(attribute='name') %}
        {% include "docs/reference/hardware/_common/_sys_index_table_body.md" %}
{%- endfor %}
{% include "docs/reference/hardware/_common/_index_table_footer.md" %}
</div>