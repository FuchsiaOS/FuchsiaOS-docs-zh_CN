{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# Fuchsia RFCs

The Fuchsia RFC process is intended to provide a consistent and transparent path
for making project-wide, technical decisions. For example, the RFC process can
be used to evolve the project roadmap and the system architecture.

The RFC process evolves over time, and can be read here in its [detailed current form]
(rfc_process.md). It is also summarized below.

## Summary of the process

- Review [criteria for requiring an RFC](#criteria).
- Socialize your idea.
- Draft your RFC using this [template](TEMPLATE.md).
- Iterate your idea with appropriate stakeholders.
- After stakeholders signoff, email <eng-council@fuchsia.dev> to prompt the Eng
  Council to decide whether to accept your RFC.
- If your RFC is accepted, a member of the Eng Council will comment on your
  change stating that the RFC is accepted, will assign the RFC a number and
  mark your change Code-Review +2. Your RFC can now be landed.

## Criteria for requiring an RFC {#criteria}

Criteria for requiring an RFC is detailed in [RFC-0001](0001_rfc_process.md).

The following kinds of changes must use the RFC process.

- Changing the project roadmap
- Adding constraints on future development
- Making project policy
- Changing the system architecture
- Delegating decision-making authority
- Escalations

In addition, changes in the source directories:

- `/zircon`
- `/src/zircon`
- `/src/bringup`

that meet the following criteria must use the RFC process as described in
[RFC0006: Addendum of the RFC Process for Zircon](0006_addendum_to_rfc_process_for_zircon.md).

- Adding or removing Zircon system interfaces.
- Changing resource handling behaviors.
- Modifying isolation guarantees.
- Significant changes of performance or memory use.
- Favoring a single platform.
- Adding or Downgrading support for a platform.
- New build configurations.

## Process to submit an RFC

Once you are familiarized with the RFC guidelines and area ready to send
an RFC proposal for review, see [Creating a RFC](create_rfc.md).

## Proposals

### Active RFCs

[Gerrit link](https://fuchsia-review.googlesource.com/q/dir:docs/contribute/governance/rfcs+is:open)

### Finalized RFCs

<div class="form-checkbox">
<devsite-expandable id="rfc-area">
  <h4 class="showalways">RFC area</h4>
<form id="filter-checkboxes-reset">
  {% for area in areas %}
    {% set found=false %}
    {% for rfc in rfcs %}
        {% for rfca in rfc.area %}
          {% if rfca == area %}
            {% set found=true %}
          {% endif %}
        {% endfor %}
    {% endfor %}
    {% if found %}
      <div class="checkbox-div">
        <input type="checkbox" id="checkbox-reset-{{ area|replace(" ", "-") }}" checked>
        <label for="checkbox-reset-{{ area|replace(" ", "-") }}">{{ area }}</label>
      </div>
    {% endif %}
  {% endfor %}
  <br>
  <br>
  <button class="select-all">Select all</button>
  <button class="clear-all">Clear all</button>
  <hr>
  <div class="see-rfcs">
    <div class="rfc-left">
      <p><a href="#accepted-rfc">Accepted RFCs</a></p>
    </div>
    <div class="rfc-right">
      <p><a href="#rejected-rfc">Rejected RFCs</a></p>
    </div>
  </div>
</form>
</devsite-expandable>

<a name="accepted-rfc"><h3 class="hide-from-toc">Accepted</h3></a>
{% include "docs/contribute/governance/rfcs/_common/_index_table_header.md" %}
{% for rfc in rfcs %}
    {% if rfc.status == "Accepted" %}
        {% include "docs/contribute/governance/rfcs/_common/_index_table_body.md" %}
    {% endif %}
{% endfor %}
{% include "docs/contribute/governance/rfcs/_common/_index_table_footer.md" %}

<a name="rejected-rfc"><h3 class="hide-from-toc">Rejected</h3></a>
{% include "docs/contribute/governance/rfcs/_common/_index_table_header.md" %}
{% for rfc in rfcs %}
    {% if rfc.status == "Rejected" %}
        {% include "docs/contribute/governance/rfcs/_common/_index_table_body.md" %}
    {% endif %}
{% endfor %}
{% include "docs/contribute/governance/rfcs/_common/_index_table_footer.md" %}

{# This div is used to close the filter that is initialized above #}
</div>
