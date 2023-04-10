{% macro pencil_edit_macro (input) -%}
  {%- if input != "" %}
    {%- setvar query "&searchAndJump=- key: " %}
    <div class="pencil-edit">
      <a href="{{ fuchsia_editor }}{{ problems_file }}{{query}}&quot;{{input}}&quot;" title="Edit the {{input}} problem"><span class="material-icons" style="font-size: 18px">edit</span></a>
    </div>
  {%- else %}
    <div class="pencil-edit">
      <a href="{{ fuchsia_editor }}{{ problems_file }}" title="Edit the problem entry"><span class="material-icons" style="font-size: 18px">edit</span></a>
    </div>
  {%- endif %}
{%- endmacro %}