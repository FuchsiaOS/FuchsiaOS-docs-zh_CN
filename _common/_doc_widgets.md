<!-- Glossary specific widgets -->

{% set glossary | yamlloads %}
{% include "docs/glossary/_glossary.yaml" %}
{% endset %}

{% set fuchsia_editor = "https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=" %}
{% set glossary_file = "docs/glossary/_glossary.yaml" %}

{% setvar pencil_edit %}
<div class="pencil-edit">
  {% if item is defined %}
    <a href="{{ fuchsia_editor }}{{ glossary_file }}&searchAndJump=- term: &quot;{{item.term}}&quot;" title="Edit the glossary"><span class="material-icons" style="font-size: 18px">edit</span></a>
  {% else %}
    <a href="{{ fuchsia_editor }}{{ glossary_file }}" title="Edit the glossary"><span class="material-icons" style="font-size: 18px">edit</span></a>
  {% endif %}
{% endsetvar %}

{% setvar glossary_edit %}
<div class="edit-buttons">
  <div class="edit-glossary">
    <p><img src="https://fonts.gstatic.com/s/i/googlematerialicons/edit/v6/googblue-24dp/1x/gm_edit_googblue_24dp.png" class="inline-icon" alt=""> <a href="{{ fuchsia_editor }}{{ glossary_file }}">Edit the glossary</a></p>
  </div>
</div>
{% endsetvar %}

{% comment %}
Help text glossary macros
{% endcomment %}

{% macro glossary_simple (input, title, notclickable) -%}
  {% set match=false %}
  {% for item in glossary %}
    {% if item.term|replace('-', ' ')|lower ==  input|replace('-', ' ')|lower and item.short_description != '' %}
      {% if title is defined %}
        {% if notclickable|lower == "notclickable" %}
          <abbr data-title="{{ item.term }}: {{ item.short_description|striptags }}">{{ title }}</abbr>
          {% set match=true %}
        {% else %}
          <a href="/docs/glossary.md#{{ item.term|replace(' ', '-')|lower }}"><abbr data-title="{{ item.term }}: {{ item.short_description|striptags }}">{{ title }}</abbr></a>
          {% set match=true %}
        {% endif %}
      {% else %}
        {% if notclickable|lower == "notclickable" %}
          <abbr data-title="{{ item.term }}: {{ item.short_description|striptags }}">{{ item.term }}</abbr>
          {% set match=true %}
        {% else %}
          <a href="/docs/glossary.md#{{ item.term|replace(' ', '-')|lower }}"><abbr data-title="{{ item.term }}: {{ item.short_description|striptags }}">{{ item.term }}</abbr></a>
          {% set match=true %}
        {% endif %}
      {% endif %}
    {% endif %}
  {% endfor %}
  {% if not match %}
    <a href="{{ fuchsia_editor }}{{ glossary_file }}&searchAndJump=- term: &quot;{{input}}&quot;"><abbr data-title="This term does not exist in the glossary.
    Check the widget call for typos. Or, click this term to add it to the glossary.">{{ input }}</abbr><span class="material-icons" style="font-size: 18px">edit</span></a>
  {% endif %}
{%- endmacro %}

{% comment %}
Simple definition callout. Use between pargraphs.
{% endcomment %}

{% macro glossary_box (term, title) -%}
  {% set match=false %}
  {% for item in glossary %}
    {% if item.term|lower ==  term|replace('-', ' ')|lower %}
    <div style="display:flex;width:100%">
       <style>
         .pencil-edit {
           float: right;
         }
       </style>
       <aside class="key-term" style="width:100%">
       <b><a href="/docs/glossary.md#{{ item.term|replace(' ', '-')|lower }}">{{ item.term }}</a>:</b>
       {% if item.full_description != '' %}
        {{ item.full_description }}
       {% elif item.short_description != '' %}
        {{ item.short_description }}
      {% else %}
         This term exists in the glossary, but does not have any description.
         Click the pencil icon to add a description.
      {% endif %}
      {% set match=true %}
      {{ pencil_edit }}
      </aside>
  </div>
    {% endif %}
  {% endfor %}
  {% if not match %}
    <div>
        <style>
         .pencil-edit {
           float: right;
         }
       </style>
       <aside class="key-term"><b><a href="{{ fuchsia_editor }}{{ glossary_file }}&searchAndJump=- term: &quot;{{term}}&quot;"><abbr data-title="This term does not exist in the glossary.
         Check the widget call for typos. Or, click this term to add it to the glossary.">{{ term }}</abbr><span class="material-icons" style="font-size: 18px">edit</span></a></b>
       </aside>
    </div>
  {% endif %}
{%- endmacro %}

<!-- General documentation widgets -->



{% macro inline_toc () -%}
<ul>
  {% for item in tocmeta.toc %}
    {% if item.path %}
      <li><a href="{{ item.path }}">{{ item.title }}</a></li>
    {% elif item.section %}
        <li>{{ item.title }}</li>
          <ul>
      {% for sectionItem in item.section %}
          {% if sectionItem.path %}
            <li><a href="{{ sectionItem.path }}">{{ sectionItem.title }}</a></li>
          {% endif %}
      {% endfor %}
        </ul>
    {% endif %}
  {% endfor %}
</ul>
{%- endmacro %}
