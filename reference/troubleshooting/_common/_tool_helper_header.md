{# This file is used to define the objects and css style for the Fuchsia glossary #}
{%- set fuchsia_source = "docs/" %}
{%- set tools_metadata_file = "_tools.yaml" %}
{%- set problems_yaml_file = "_problems.yaml" %}
{%- set fuchsia_editor = "https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=" %}
{%- set problems_file = "docs/reference/troubleshooting/_problems.yaml" %}
{%- set tools_yaml_file = "docs/reference/troubleshooting/_tools.yaml" %}

{%- set problems | yamlloads %}
{% include "docs/reference/troubleshooting/_problems.yaml" %}
{%- endset %}

{%- set tools | yamlloads %}
{% include "docs/reference/troubleshooting/_tools.yaml" %}
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