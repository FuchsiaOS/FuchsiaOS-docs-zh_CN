{# This file is used to define the objects and css style for the Fuchsia glossary #}
{%- set fuchsia_source = "docs/" %}
{%- set glossary_metadata_file = "glossary.yaml" %}
{%- set areas_yaml_file = "_areas.yaml" %}
{%- set fuchsia_editor = "https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=" %}
{%- set glossary_file = "docs/glossary/glossary.yaml" %}
{%- set areas_yaml_file = "docs/contribute/governance/rfcs/_areas.yaml" %}

{%- set areas | yamlloads %}
{% include "docs/contribute/governance/rfcs/_areas.yaml" %}
{%- endset %}

{%- set glossary | yamlloads %}
{% include "docs/glossary/glossary.yaml" %}
{%- endset %}

<style>
.edit-buttons {
  display:inline-block;
  width:100%;
  margin-bottom: -30px;
}

.pencil-edit {
  float: right;
}
.edit-glossary {
  float: right;
}

.list {
  list-style: none;
}

.comma-list {
  display: inline;
  list-style: none;
  padding: 0px;
}

.comma-list li {
  display: inline;
}

.comma-list li::after {
  content: ", ";
}

.comma-list li:last-child::after {
    content: "";
}

.checkbox-div {
  display:inline-block;
  padding-top: 3px;
  padding-right: 2px;
  padding-bottom: 3px;
  padding-left: 2px;
}

.checkbox-div input+label {
  font-size: 80%;
}

.form-checkbox button {
  font-size: 80%;
}

.col-key {
  width:1px;white-space:nowrap;
}
</style>