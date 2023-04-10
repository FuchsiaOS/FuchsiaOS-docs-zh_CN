
{% import "docs/reference/troubleshooting/_common/_tool_helper_macros.md" as macro %}
<!-- The header above is automatically added to this file. Do not modify anything above this line. -->
<li>
  {{ macro.pencil_edit_macro(item.key) }}
  <h3 id ="{{ item.key}}"class="add-link">{{ item.key[0]|title }}{{ item.key[1:]|replace("-", " ") }}</h3>
    {%- if item.use_case|length %}
    <p>{{ item.use_case }}</p>
    {%- endif %}
    <hr>
    {%- if item.tools!= [''] %}
    <h4 id="{{ item.key}}-tools" class="add-link">Tools</h4>
      <ul>
      {%- for tool in item.tools %}
      <li>{{ tool }}</li>
        {%- for a in tools %}
          {%- if a.name == tool %}
          <p>{{ a.description }}</p>
           {%- if a.links != [''] %}
           <ul>
           {%- for key,value in a.links.items() %}
           <li><a href="{{value}}">{{key}}</a></li>
           {%- endfor %}
           </ul>
           {%- endif %}
          {%- endif %}
        {%- endfor %}
      {%- endfor %}
      </ul>
    {%- endif %}
    {%- if item.related_problems != [''] %}
      <ul>
      {%- for related in item.related_problem %}
      <li>{{ related }}</li>
      {%- endfor %}
      </ul>
    {%- endif %}
<hr>
</li>
