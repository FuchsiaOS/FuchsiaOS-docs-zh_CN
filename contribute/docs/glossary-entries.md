{% include "docs/glossary/_common/_glossary_header.md" %}

{% import 'docs/_common/_doc_widgets.md' as widgets %}

# Adding a glossary term

Fuchsia.dev uses a [glossary.yaml][glossary-yaml] file to define all Fuchsia
specific terminology.

This format allows Fuchsia to use single-source definitions to be edited in
a single location and updated throughout the documentation. These definitions
can then be used throughout documentation using documentation widgets, which
then allows you to use inline definitions or full definitions in your documents.

Note: For more information, see [documentation widgets][documentation-widgets].

Additionally, Fuchsia has a main filterable [glossary page][glossary-index].

## Add a glossary entry

To add a glossary definition, you need to edit the
[glossary.yaml][glossary-yaml] to include information about your definition:

<div>
<devsite-selector>
  <section>
    <h3>Reference</h3>
    <table class="responsive">
      <tbody>
        <tr>
          <th colspan=2>Glossary definition</th>
        </tr>
        <tr>
          <td><code>term</code></td><td><b>Required</b>
            <p>
              Define the glossary term.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>short_description</code></td><td><b>Required</b>
            <p>
              Define a short description for your glossary term. This definition
              must be short, preferably a single sentence. This definition can
              then be used to have definitions as hover-over text.
            </p>
            <p>
              You must use HTML syntax. However, if you have a simple definition
              that is a single sentence or paragraph, you can use plain text.
            </p>
            <p>
              For example, this is the definition for the term
              {{ widgets.glossary_simple ('ABI') }}.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>full_description</code></td><td><b>Optional</b>
            <p>
              Define a full for your glossary term. This description should be
              complete and fully explain the term.
            </p>
            <p>
              You must use HTML syntax, including for links.
              However, if you have a simple definition
              that is a single sentence or paragraph, you can use plain text.
            </p>
            <p>For a link, use this format:</p>
            <aside class="note">
            <b>Note:</b> Links to the glossary should use this link location format, <code>/docs/glossary/README.md#<var>term</var></code>.
            </aside>
               <pre class="prettyprint">
{% htmlescape %}<a href="{% endhtmlescape %}<var>link_location</var>{% htmlescape %}">{% endhtmlescape %}<var>link_title</var>{% htmlescape %}</a>{% endhtmlescape %}
               </pre>
          </td>
        </tr>
        <tr>
          <td><code>see_also</code></td><td><b>Optional</b>
            <p>
              Create a list of related links for the term. This can be links
              to other terms or related documentation. You must use HTML syntax.
            </p>
            <aside class="note">
            <b>Note:</b> Links to the glossary should use this link location format, <code>/docs/glossary/README.md#<var>term</var></code>.
            </aside>
            <ul>
              <li>For a single link, use this format:
                <pre class="prettyprint">
{% htmlescape %}['<a href="{% endhtmlescape %}<var>link_location</var>{% htmlescape %}">{% endhtmlescape %}<var>link_title</var>{% htmlescape %}</a>']{% endhtmlescape %}
               </pre>
              </li>
              <li>For multiple links, use this format:
                 <pre class="prettyprint">
{% htmlescape %}['<a href="{% endhtmlescape %}<var>link_location</var>{% htmlescape %}">{% endhtmlescape %}<var>link_title</var>{% htmlescape %}</a>'{% endhtmlescape %},
{% htmlescape %}['<a href="{% endhtmlescape %}<var>link_location2</var>{% htmlescape %}">{% endhtmlescape %}<var>link_title2</var>{% htmlescape %}</a>']{% endhtmlescape %}
                 </pre>
              </li>
            </ul>
          </td>
        </tr>
        <tr>
          <td><code>related_guides</code></td><td><b>Optional</b>
            <p>
              Create a list of related guides for the term. This should
              only be links to guides located in <code>//docs/development/</code>. You must
              use HTML syntax.
            </p>
            <ul>
              <li>For a single link, use this format:
               <pre class="prettyprint">
{% htmlescape %}['<a href="{% endhtmlescape %}<var>guide_location</var>{% htmlescape %}">{% endhtmlescape %}<var>guide_title</var>{% htmlescape %}</a>']{% endhtmlescape %}
               </pre>
              </li>
              <li>For multiple links, use this format:
                  <pre class="prettyprint">
{% htmlescape %}['<a href="{% endhtmlescape %}<var>guide_location</var>{% htmlescape %}">{% endhtmlescape %}<var>guide_title</var>{% htmlescape %}</a>'{% endhtmlescape %},
{% htmlescape %}['<a href="{% endhtmlescape %}<var>guide_location2</var>{% htmlescape %}">{% endhtmlescape %}<var>guide_title2</var>{% htmlescape %}</a>']{% endhtmlescape %}
                 </pre>
              </li>
            </ul>
          </td>
        </tr>
        <tr>
          <td><code>area</code></td><td><b>Required</b>
            <p>
              Define a list of the areas that your glossary term pertains to.
              These areas make the glossary terms filterable on the
              <a href="/docs/glossary">glossary page</a>.
              You must use HTML syntax.
            </p>
            <ul>
              <li>For a single area, use this format ['area'].</li>
              <li>For multiple areas, use this format ['area1', ...]</li>
            </ul>
            <devsite-expandable>
              <p>Valid areas</p>
              <a name="valid-areas"></a>
              <aside class="note"><b>Note:</b> If you think there should be additional
                areas, <a class="external" href="{{ fuchsia_editor }}{{ areas_yaml_file }}">suggest a new area.</a></aside>
              <a href="#valid-areas" class="expand-control once">Valid areas</a>
              <ul>
                {% for area in areas %}
                <li><code>{{ area }}</code></li>
                {% endfor %}
               </ul>
            </devsite-expandable>
          </td>
        </tr>
  </tbody>
</table>
</section>
  <section>
    <h3>Sample</h3>
<pre class="devsite-click-to-copy">
- term: 'ABI'
  short_description: 'The binary-level interface to the system.'
  full_description: '{% htmlescape %}The <a href="/docs/concepts/packages/system.md">Application Binary Interface</a> (ABI)
  for a system is the binary-level interface to the system. Typically you don''t
  write software that uses the system ABI directly. Instead, you write software
  against the system API. When the software is compiled, the binary artifact
  created by the compiler interfaces with the system through the ABI.
  Changes to the system ABI may require you to recompile your source code to
  account for the changes in the ABI.{% endhtmlescape %}'
  see_also: ['{% htmlescape %}<a href="/docs/glossary#ABI">ABI</a>',
             '<a href="/docs/glossary#storage-capability">Storage capability</a>{% endhtmlescape %}']
  related_guides: ['{% htmlescape %}<a href="/docs/development/tracing/tutorial/registering-a-trace-provider.md">Registering a trace provider</a>',
                   '<a href="/docs/development/hardware/paving.md">Installing Fuchsia on a device</a>{% endhtmlescape %}']
  area: ['System', 'General']
</pre>
  </section>
</devsite-selector>
</div>

Once you have created the entry for your glossary entry and submitted the Gerrit
change, your glossary term will be visible on the [glossary page][glossary-index]
and usable as a [documentation widget][documentation-widgets].

<!-- xrefs -->

[glossary-yaml]: /docs/glossary/glossary.yaml
[glossary-index]: /docs/glossary/README.md
[documentation-widgets]: /docs/contribute/docs/widgets.md



