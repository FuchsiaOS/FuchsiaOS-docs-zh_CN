{% include "docs/glossary/_common/_glossary_header.md" %}

{% import 'docs/_common/_doc_widgets.md' as widgets %}

{% comment %}
# This is used to do the initial import statement in the doc due to a copybara limitation
[ABI][glossary.ABI]
[glossary.ABI]: glossary/README.md#ABI
{% endcomment %}

# Documentation widgets

Documentation widgets are a way to simplify and single source information for
use in the documentation.

The Fuchsia.dev documentation widgets are created with Jinja2 macros and are
also supported in markdown format. The transformation of the widgets from
markdown to an actual Jinja2 macro happens before the page is published on
fuchsia.dev. For more information on Jinja2 macros, see
[macros][jinja-macros]{: .external}.

All of the documentation widgets are defined in
[//docs/_common/_doc_widgets.md][doc-widgets].

## Prerequisites: (only for HTML/Jinja2) {#widgets-prereq}

Note: This is not required if you use markdown syntax. You only need to use an
import statement when using the HTML/Jinja2 version of the widgets.

Before you can use the documentation widgets in HTML/Jinja2, you must import
them into your markdown (.md) file. At the top of your file, specify the
following:

```none
{% verbatim %}
{% import 'docs/_common/_doc_widgets.md' as widgets %}
{% endverbatim %}
```

## General widgets

### inline_toc

Creates a bulleted list of a table of contents (TOC) based on a `_toc.yaml` file.

#### Examples

* Bulleted list of a TOC:

  * {Rendered}

     {% set tocmeta | yamlloads %}
     {% include "docs/contribute/docs/_toc.yaml" %}
     {% endset %}
     {{ widgets.inline_toc () }}

  * {HTML/Jinja2}

     {% verbatim %}
     {% set tocmeta | yamlloads %}

     {% include "docs/contribute/docs/_toc.yaml" %}

     {% endset %}

     {{ widgets.inline_toc () }}

     {% endverbatim %}

#### Usage

Note: This widget does not have a markdown equivalent and must also import the
widgets on the page. For more information on how to import widgets, see
[Prerequisites: (only for HTML/Jinja2)](#widgets-prereq).

Due to limitations, you cannot specify the `_toc.yaml` file as a parameter of
the widget.

```none
{% verbatim %}
{% set tocmeta | yamlloads %}
{% include "<var>_toc_file</var>.yaml" %}
{% endset %}
{{ widgets.inline_toc () }}
{% endverbatim %}
```

#### Parameters

This widget does not use parameters, make sure to specify the
[prerequisites](#widgets-prereq) before using this widget.

## Glossary

These widgets are specifically created to work with the glossary terms defined
in [/glossary/_glossary.yaml][glossary-yaml].


In order to enable over-hover definitions, you must use one of the syntaxes listed below. The
markdown version is recommended. Using any other syntax will result in simple links without
over-hover definitions.

For more information on adding a term to the glossary, see
[Adding glossary entries][glossary-add].

### glossary_simple

Creates a hover-over definition of the `short_description` of a term with
a link to the glossary term. Optionally, this term can be unclickable.

#### Examples

* Clickable over-hover definition:

  * {Rendered}

     Definition of [ABI](glossary/README.md#ABI).

* Non-clickable over-hover definition:

  Note: This option is only available when using the Jinja2 syntax.

  * {Rendered}

     Definition of {{ widgets.glossary_simple ('ABI', 'ABI', 'notClickable')}}.

#### Usage

There are several ways of using this widget:

* {Markdown}

  * Xref link (preferred):

  <pre>{% verbatim %}[<var>display_name</var>][glossary.<var>term</var>]{% endverbatim %}</pre>

  Alternatively, you don't need to specify `{{"<var>display_name</var>"}}`, which lets the actual term
  be used as the `{{"<var>display_name</var>"}}`:

  <pre>{% verbatim %}[glossary.<var>term</var>]{% endverbatim %}</pre>

  For both formats, you must define the Xref at the bottom of the markdown file. For example:

  <pre>{% verbatim %}[glossary.<var>display_name</var>]: glossary/README.md#<var>term</var>{% endverbatim %}</pre>

  * Inline link:

  <pre>{% verbatim %}[<var>display_name</var>](glossary/README.md#<var>term</var>){% endverbatim %}</pre>

  * Inline link (shortened):

  <pre>{% verbatim %}[<var>display_name</var>](glossary#<var>term</var>){% endverbatim %}</pre>

* {HTML/Jinja2}

  Note: To use the glossary widgets in any HTML markup, you muse use the JINJA2 syntax.
  This is useful when creating HTML tables. Also, make sure to follow the
  [Prerequisites: (only for HTML/Jinja2)](#widgets-prereq).

  ```none
  {% verbatim %}{{ widgets.glossary_simple ('<var>term</var>', '<var>display_name</var>', '<var>notClickable</var>')}}{% endverbatim %}
  ```


#### Parameters

Note: When using the widgets in Jinja2 there are additional parameters available.

* {Markdown}

  <table class="responsive">
    <tbody>
      <tr>
        <th colspan=2>Parameters</th>
      </tr>
      <tr>
        <td><code>display_name</code></td>
        <td><b>Required</b><br><p>Specify the text in your markdown file that will have
        hover over text.</p>
        <p>Not required when using the xref syntax of
        {% verbatim %}[glossary.<var>term</var>]{% endverbatim %}. In that case,
        the glossary term is used as the <var>display_name</var>.</p></td>
      </tr>
      <tr>
        <td><code>term</code></td>
        <td><b>Required</b><br>Specify a term that is defined
          in the <a href="{{fuchsia_editor }}{{ glossary_file }}">
          _glossary.yaml</a> file.</td>
      </tr>
    </tbody>
  </table>

* {HTML/Jinja2}

  <table class="responsive">
    <tbody>
      <tr>
        <th colspan=2>Parameters</th>
      </tr>
      <tr>
        <td><code>term</code></td>
        <td><b>Required</b><br>Specify a term that is defined
          in the <a href="{{fuchsia_editor }}{{ glossary_file }}">
          _glossary.yaml</a> file.</td>
      </tr>
      <tr>
        <td><code>display_name</code></td>
        <td><b>Optional</b><br>Specify the text in your markdown file that will have
        hover over text.
      </tr>
      <tr>
        <td><code>notClickable</code></td>
        <td><b>Optional</b><br>Required if using <code>display_name</code>Determines
        if the term gets a link to the full glossary. If this is not
        specified, the term will become clickable and have a link to its glossary
        entry.</td>
      </tr>
    </tbody>
  </table>

### glossary_box

Creates a definition box of the `full_description` of a term. If the term
does not have a `full_description`, the `short_description` is used.

The definition box also displays an edit button for contributors to edit the
glossary.

#### Examples

* Definition box:

  * {Rendered}

     [ABI](glossary/README.md?style=box#ABI)

#### Usage

Note: The definition box should not be used inside of other markdown elements
such as paragraphs. It should only be used between
other elements such as pagraphs, headings, lists, etc... If you use the definition
box inside a paragraph it will cause text around it to get wrapped.

There are several ways of using this widget:

* {Markdown}

  * Xref link (preferred):

    <pre>
    {% verbatim %}[<var>display_name</var>][glossary.box.<var>term</var>]{% endverbatim %}
    </pre>

    Then, you must define the Xref at the bottom of the markdown file. For example:

    <pre>{% verbatim %}[glossary.box.<var>display_name</var>]: glossary/README.md#<var>term</var>{% endverbatim %}</pre>

  * Inline link:

    <pre>
    {% verbatim %}[<var>display_name</var>](glossary/README.md?style=box#<var>term</var>){% endverbatim  %}
    </pre>

  * Inline link (shortened):

    <pre>
    {% verbatim %}[<var>display_name</var>](glossary?style=box#<var>term</var>){% endverbatim %}
    </pre>


* {HTML/Jinja2}

  Note: The <var>display_name</var> parameter does not do anything and is only
  defined to ensure consistency with the usage of the simple glossary widgets. Also,
  make sure to follow the [Prerequisites: (only for HTML/Jinja2)](#widgets-prereq).

  ```none
  {% verbatim %}{{ widgets.glossary_box ('<var>term</var>', '<var>display_name</var>') }}{% endverbatim %}
  ```

#### Parameters

Note: The parameters for markdown and Jinja2 are the same.

<table class="responsive">
  <tbody>
    <tr>
      <th colspan=2>Parameters</th>
    </tr>
    <tr>
      <td><code>term</code></td>
      <td><b>Required</b><br>Specify a term that is defined
        in the <a href="{{fuchsia_editor }}{{ glossary_file }}">
        _glossary.yaml</a> file.</td>
    </tr>
    <tr>
        <td><code>display_name</code></td>
        <td><b>Required</b><br>This parameter is required to prevent errors, but
        does not do anything.
    </tr>
  </tbody>
</table>

<!-- xrefs -->

[doc-widgets]: {{fuchsia_editor }}docs/_common/_doc_widgets.md
[glossary-yaml]: {{fuchsia_editor }}{{ glossary_file }}
[jinja-macros]: https://jinja.palletsprojects.com/en/2.11.x/templates/#macros
[glossary-add]: contribute/docs/glossary-entries.md
