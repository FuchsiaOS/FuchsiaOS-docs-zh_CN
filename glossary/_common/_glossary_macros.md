{% macro pencil_edit_macro (input) -%}
  {%- if input != "" %}
    {%- setvar query "&searchAndJump=- term: " %}
    <div class="pencil-edit">
      <a href="{{ fuchsia_editor }}{{ glossary_file }}{{query}}&quot;{{input}}&quot;" title="Edit the glossary"><span class="material-icons" style="font-size: 18px">edit</span></a>
    </div>
  {%- else %}
    <div class="pencil-edit">
      <a href="{{ fuchsia_editor }}{{ glossary_file }}" title="Edit the glossary"><span class="material-icons" style="font-size: 18px">edit</span></a>
    </div>
  {%- endif %}
{%- endmacro %}
