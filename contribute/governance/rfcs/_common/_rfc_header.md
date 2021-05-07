{# This file is used to define the objects and css style for RFC pages #}
{% set gerrit_profile = "https://fuchsia-review.googlesource.com/q/owner:" %}
{% set gerrit_change_url = "https://fuchsia-review.googlesource.com/c/fuchsia/+/" %}
{% set fuchsia_source_tree = "https://fuchsia.googlesource.com/fuchsia/+/main/" %}
{% set fuchsia_editor = "https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=" %}
{% set issue_url = "https://fxbug.dev/" %}
{% set rfcs_dir = "docs/contribute/governance/rfcs/" %}
{% set rfcs_metadata_file = "_rfcs.yaml" %}
{% set eng_council_yaml_file = "_eng_council.yaml" %}
{% set areas_yaml_file = "_areas.yaml" %}

{% set rfcs | yamlloads %}
{% include "docs/contribute/governance/rfcs/_rfcs.yaml" %}
{% endset %}

{% set areas | yamlloads %}
{% include "docs/contribute/governance/rfcs/_areas.yaml" %}
{% endset %}

{% set eng_council | yamlloads %}
{% include "docs/contribute/governance/rfcs/_eng_council.yaml" %}
{% endset %}

{% if rfcid %}
    {% for _rfc in rfcs %}
        {% if _rfc.name == rfcid %}
            {% set rfc=_rfc %}
            {% include "docs/contribute/governance/rfcs/_common/_rfc_metadata.md" %}
            {% set found=true %}
        {% endif %}
    {% endfor %}
    {% if not found %}
      <h2> ERROR! Invalid RFC number: {{ rfcid }} </h2>
      There must be an entry with "name: {{ rfcid }}" in file {{ rfcs_dir }}{{ rfcs_metadata_file }}
    {% endif %}
{% endif %}

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

.checkbox-div input+label {
  font-size: 80%;
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

.rfc-left {
  float: left;
  margin-left: 20%;
}

.rfc-right {
  float: right;
  margin-right: 20%;
}
</style>

