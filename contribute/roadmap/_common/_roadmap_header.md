{# This file is used to define the objects and css style for driver pages #}
{%- set url_qualifier = "http://" %}

{%- set areas | yamlloads %}
{% include "docs/contribute/governance/rfcs/_areas.yaml" %}
{%- endset %}

<style>
.types {
  font-size: 80%;
}

.list-areas {
  list-style-type: none;
}

.list-items {
  margin-left: 1em;
}

.workstream {

  margin: 0;
  padding: 0;
}

.cat {
  font-size: 80%;
  margin: 0;
  padding: 0;
}

.bug {
  font-size: 80%;
}

hr.item-divider {
  margin: 0em;
  border-width: 1px;
}

.list li, li p {
  margin: 5px 3px 0 0px;

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

</style>