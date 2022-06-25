
{% import "docs/glossary/_common/_glossary_macros.md" as macro %}
<!-- The header above is automatically added to this file. Do not modify anything above this line. -->
<li>
  {{ macro.pencil_edit_macro(item.term) }}
  <h3 class="add-link">{{ item.term }}</h3>
    {%- if item.short_description|length %}
    <p>{{ item.short_description }}</p>
      {%- if item.full_description|length %}
      <devsite-expandable>
        <a href="#{{ item.term }}-full" class="expand-control once">Full description</a>
        <hr>
        <p>{{ item.full_description }}</p>
      </devsite-expandable>
      {%- endif %}
    {%- else %}
      {%- if item.full_description|length %}
         {{ item.full_description }}
      {%- endif %}
    {%- endif %}
  {%- if item.see_also!= [''] or item.related_guides!= [''] %}
    <hr>
    <devsite-expandable>
    <a href="#{{ item.term }}-also" class="expand-control once">Related links</a>
    {%- if item.see_also!= [''] %}
      <h4>Information related to {{ item.term }}</h4>
        <ul class="comma-list">
        {%- for see in item.see_also %}
        <li>{{ see }}</li>
        {%- endfor %}
      </ul>
    {%- endif %}
    {%- if item.related_guides!= [''] %}
      <h4>Guides related to {{ item.term }}</h4>
        <ul class="comma-list">
        {%- for guide in item.related_guides %}
        <li>{{ guide }}</li>
        {%- endfor %}
      </ul>
  {%- endif %}
  </devsite-expandable>
  {%- endif %}
  {%- if item.area!= [''] %}
  <!--
    <ul class="comma-list">
      {% for area in item.area %}
      <li>{{ area }}</li>
      {% endfor %}
    </ul>
  -->
  {%- endif %}
<hr>
</li>
