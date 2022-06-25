{# This file is used to define the objects and css style for driver pages #}
{%- set gerrit_profile = "https://fuchsia-review.googlesource.com/q/owner:" %}
{%- set gerrit_change_url = "https://fuchsia-review.googlesource.com/c/fuchsia/+/" %}
{%- set fuchsia_source_tree = "https://fuchsia.googlesource.com/fuchsia/+/main/" %}
{%- set fuchsia_editor = "https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=" %}
{%- set issue_url = "https://fxbug.dev/" %}
{%- set cs_url = "https://cs.opensource.google/fuchsia/fuchsia/+/main:" %}
{%- set fuchsia_source_tree_change = "https://cs.opensource.google/fuchsia/fuchsia/+/" %}
{%- set drivers_dir = "reference/drivers/" %}
{%- set drivers_metadata_file = "all_drivers_doc.yaml" %}
{%- set areas_yaml_file = "_drivers_areas.yaml" %}
{%- set info_icon = '<span class="material-icons" style="font-size: 1.1em;color:#007b83;vertical-align: top;">info</span>' %}
{%- set sys_config_page = "/docs/reference/hardware/support-system-config#" %}

{%- set drivers | yamlloads %}
{% include "reference/drivers/all_drivers_doc.yaml" %}
{%- endset %}

{%- set areas | yamlloads %}
{% include "docs/reference/hardware/_drivers_areas.yaml" %}
{%- endset %}

{%- set epitaphs | yamlloads %}
{% include "docs/reference/hardware/_drivers_epitaphs.yaml" %}
{%- endset %}

<style>
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

table {
  text-overflow: ellipsis;
}


.checkbox-div {
  display:inline-block;
  padding-top: 3px;
  padding-right: 2px;
  padding-bottom: 3px;
  padding-left: 2px;
}

.checkbox-div input {
  margin-right: 1px;
  margin-left: 5px;
}

.checkbox-div input+label {
  font-size: 90%;
}

.form-checkbox button {
  font-size: 80%;
}

.col-key {
  width:1px;white-space:nowrap;
}

.note {

}
.edit-buttons {
  display:inline-block;
  width:100%;
}

.edit-buttons-left {
  float: left;
  margin-left: 20%;
}

.edit-buttons-right {
  float: right;
  margin-right: 20%;
}

.see-rfcs {
  display:inline-block;
  width:100%;
}

</style>