
{% import "docs/reference/troubleshooting/_common/_tool_helper_macros.md" as macro %}
<!-- The header above is automatically added to this file. Do not modify anything above this line. -->
<li>
  {{ macro.pencil_edit_macro(item.name) }}
  <h3 id ="{{ item.name}}"class="add-link">{{ item.name[0]|title }}{{ item.name[1:]|replace("-", " ") }}</h3>
    {%- if item.description|length %}
    <p>{{ item.description }}</p>
    {%- endif %}
    <hr>
    <p>Problems that {{item.name}} can solve:</p>
      <ul>
      {%- for problem in problems %}
      {%- for tool in problem.tools if problem.tools != [''] %}
        {%- if tool == item.name %}
        <li>{{problem.use_case}}</li>
        {%- endif %}
      {%- endfor %}
      {%- endfor %}
      </ul>
    <hr>
    {%- if item.links != [''] %}
      <ul>
      {%- for key, value in item.links.items() %}
      <li><a href="{{value}}">{{key}}</a></li>
      {%- endfor %}
      </ul>
    {%- endif %}
    {%- if item.related != [''] and item.related|length %}
      <h4 id="{{item.name}}-related-tools" class="add-link">Related tools</h4>
      <ul>
      {%- for rel in item.related %}
      <li>{{ rel }}</li>
      {%- endfor %}
      </ul>
    {%- endif %}
<hr>
</li>
