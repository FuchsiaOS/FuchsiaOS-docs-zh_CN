{# Imports JSONs to generate content #}

{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# Creating an RFC

Once you are familiarized with the
[RFC (request for comments) process](README.md) and criteria, you may
want to create a proposal for the [Fuchsia Eng Council][eng-council] to review.

To create an RFC, you need to create a Gerrit change that contains at least the
following three files:

Note: You may have additional files if you are including images in your RFC.

* [Create metadata](#create-metadata)
* [Create a markdown file](#create-markdown)
* [Create a _toc.yaml entry](#toc)

Note: You can use the `fx rfc` tool that automates these three steps. This
tool helps you to interactively populate the required RFC metadata fields and
creates a blank RFC markdown file using the [template][rfc-template].

## Create metadata {#create-metadata}

The metadata of an RFC defines many values that are used to populate the RFC
information for the [RFC overview][rfc-overview].

To identify your RFC, you need to edit the
[docs/contribute/governance/rfcs/_rfcs.yaml][rfc-yaml] to include information about your RFC.

<p>For your RFC, you need to include the following information:</p>

<div>
<devsite-selector>
  <section>
    <h3>Reference</h3>
    <table class="responsive">
      <tbody>
        <tr>
          <th colspan=2>RFC metadata</th>
        </tr>
        <tr>
          <td><code>name</code></td><td><b>Required</b>
            <p>
              Define an RFC name. When you first create an RFC, you should use
              <code>RFC-NNNN</code>. After your RFC is approved or rejected, the
              <a href="../eng_council.md">Fuchsia Eng Council</a> assigns
              you a unique name to use.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>title</code></td><td><b>Required</b>
            <p>
              Define a title for your RFC. This title must be short and
              explain your RFC in a few words.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>short_description</code></td><td><b>Required</b>
            <p>
              Define a short description for your RFC. This description must be short
              and should not be longer than a few sentences.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>authors</code></td><td><b>Required</b>
            <p>
              Create a list of the authors for the RFC.
            </p>
            <ul>
              <li>For a single author, use this format ["myemail@example.com"].</li>
              <li>For multiple authors, use this format ["myemail@example.com", ...]</li>
            </ul>
          </td>
        </tr>
        <tr>
          <td><code>file</code></td><td><b>Required</b>
            <p>
              Define the filename for the markdown file of your RFC. This should
              include the numerical part of the <code>name</code> of your RFC and an
              identifier based on the RFC title.
            </p>
            <p>
              Until you receive an RFC name, use <code>NNNN</code> for the
              numerical part, for example, <code>NNNN_file</code>.
              Once your RFC is reviewed you will receive an RFC name. For example,
              you could use a <code>file</code> of
              <code>0001_rfc_process.md</code> for an RFC with an
              <code>id</code> of <code>0001</code> and a title of
              <code>Fuchsia Request for Comments (RFC) process</code>.
            </p>
          </td>
        </tr>
        <tr>
          <td><code>area</code></td><td><b>Required</b>
            <p>
              Define a list of the areas that your RFC affects.
            </p>
            <ul>
              <li>For a single area, use this format `['area']`.</li>
              <li>For multiple areas, use this format `['area1', ...]`.</li>
            </ul>
            <devsite-expandable>
              <p>Valid areas</p>
              <a name="valid-areas"></a>
              <aside class="note"><b>Note:</b> If you think there should be additional
                areas, <a class="external" href="{{ fuchsia_editor }}{{ rfcs_dir }}{{ areas_yaml_file }}">suggest a new RFC area.</a></aside>
              <a href="#valid-areas" class="expand-control once">Valid areas</a>
              <ul>
                {% for area in areas %}
                <li><code>{{ area }}</code></li>
                {% endfor %}
               </ul>
            </devsite-expandable>
          </td>
        </tr>
        <tr>
          <td><code>issue</code></td><td><b>Required</b>
            <p>
              Define a list of the issue that tracks your RFC on <a href="{{ issue_url }}">{{ issue_url }}</a>.
              In case you don't have an `issue` number, you can leave this value
              blank, <code>[]</code>.
            </p>
            <ul>
              <li>For a single issue, use this format ['issue'].</li>
              <li>For multiple issues, use this format ['issue1', ...]</li>
            </ul>
          </td>
        </tr>
        <tr>
          <td><code>gerrit_change_id</code></td><td><b>Required</b>
            <p>
              The <code>id</code> of your gerrit change that contains your rfc. These
              are the final digits of the URL of your gerrit change. For example,
              if the URL of your change is <a href="https://fuchsia-review.googlesource.com/c/fuchsia/+/366393">https://fuchsia-review.googlesource.com/c/fuchsia/+/366393</a>,
              use a <code>gerrit_change_id</code> of <code>366393</code>.
            </p>
          </td>
        </tr>
      <tr>
        <td><code>submitted</code></td><td><b>Required</b>
          <p>
            Define the date when you first created the change for your RFC in
            a <code>year-month-day</code> format.
          </p>
        </td>
      </tr>
        <tr>
          <td><code>status</code></td><td><b>Required</b>
            <p>
              Define the status of your RFC. When you first submit an RFC, this
              value should be <code>Pending</code>. After your RFC is reviewed,
              the status will be changed to <code>Accepted</code> or
              <code>Rejected</code>.
            </p>
          </td>
        </tr>
        <tr>
      <td><code>reviewers</code></td><td><b>Required once approved or rejected</b>
        <p>
          Define the reviewers of the RFC from the Gerrit change.
        </p>
      </td>
    </tr>
    <tr>
      <td><code>consulted</code></td><td><b>Required once approved or rejected</b>
        <p>
        Stakeholders who were consulted about this RFC, but whose +1 is not required.
        </p>
      </td>
    </tr>
    <tr>
      <td><code>reviewed</code></td><td><b>Required</b>
        <p>
          Define the date when you received the decision from the
          <a href="../eng_council.md">Fuchsia Eng Council</a> as to if your RFC
          is accepted or rejected.
        </p>
      </td>
    </tr>
  </tbody>
</table>
</section>
  <section>
    <h3>Sample</h3>
<pre class="prettyprint">
- name: 'RFC-NNNN'
  title: 'zx_channel_iovec_t support for zx_channel_write and zx_channel_call'
  short_description: 'This RFC introduces a new mode to zx_channel_write and zx_channel_call that copies input data from multiple memory regions rather than from a single contiguous buffer.'
  authors: ['bprosnitz@google.com']
  file: '0010_channel_iovec.md'
  area: ['Zircon']
  issue: ['60623']
  gerrit_change_id: ['433621']
  status: ''
  reviewers: []
  submitted: '2020-09-25'
  reviewed: ''
</pre>
  </section>
</devsite-selector>
</div>

Once you have created the metadata for your RFC, you are ready to write
a markdown file with the information for your RFC.

## Create a markdown file {#create-markdown}

Once you have filled out your RFC metadata and a `name` for your RFC, you can
create a markdown file for your RFC.

To create a markdown file:

1. Create a new markdown file from the [RFC template][rfc-template]. This file
   must be named based on the `file` value that you added to the metadata file.

1. When you edit the file with a text editor, you need to edit the first line
   of the template that contains a `rfcid` variable with the `name` of your
   RFC. For example, if your `name` is `"RFC-NNNN"`, the first line looks like
   the following:

   ```
   {% verbatim %}
   {% set rfcid = "RFC-NNNN" %}
   {% endverbatim %}
   ```

1. Fill out the different sections for your RFC, after
   the ## Summary header.

Note: If you are adding images, you should name the images based on the `file`
name of your RFC. For example, <code><var>file</var>-fig_1.png</code>.

Once you have finished filling out the markdown file, you are ready to create
an entry for the RFC in the table of contents.

## Create a _toc.yaml entry {#toc}

Once you have created the metadata for your RFC and created the markdown file,
you are ready to list your RFC in the TOC (table of contents).

Note: For more information about `_toc.yaml` files, see
[Updating site navigation and TOC files][toc-ref].

To add a TOC entry:

1. Edit the [//docs/contribute/governance/rfcs/_toc.yaml][toc-ref] file.
1. Add your RFC entry below the last entry in the following format.

<div>
<devsite-selector>
  <section>
    <h3>Reference</h3>
<pre class="prettyprint">
   - title: "{{ "<var>" }}name{{ "</var>" }}: {{ "<var>" }}title{{ "</var>" }}"
     path: /docs/contribute/governance/rfcs/{{ "<var>" }}file{{ "</var>" }}.md
</pre>
</section>

  <section>
    <h3>Sample</h3>
<pre class="prettyprint">
- title: "RFC-0001: RFC Process"
  path: /docs/contribute/governance/rfcs/0001_rfc_process.md
</pre>
  </section>
</devsite-selector>
</div>

   Where `name`, `title`, and `file` are the values that you defined in the
   metadata file.

Once you have created a TOC entry, you are now ready to submit your RFC for review.

## Send a change for review {#review}

Once you have edited the metadata file, created a markdown file, and added a TOC
entry, you are ready to create a change for review.

The first line of your change's commit message must be `[rfc]`, followed by the
RFC's name. The body of the commit message may also include your RFC's short
description. For example:

```md
[rfc] zx_channel_iovec_t support for zx_channel_write and zx_channel_call

This RFC introduces a new mode to zx_channel_write and zx_channel_call that
copies input data from multiple memory regions rather than from a single
contiguous buffer.
```

Mail your change to your initial set of reviewers. The Fuchsia Eng Council will
be automatically notified about the RFC's creation.

Note: After you submit your change, update the metadata to include the assigned
Gerrit change id and submit a patch to your change.

For more information, on creating a change in the Fuchsia project, see
[Contribute changes].

## Change the status of an RFC proposal {#rfc-change-status}

Once your change has gone through the review process, the [Fuchsia Eng Council][eng-council]
will let you know if your proposal was accepted or rejected. Regardless of the
final status of the proposal, you will be assigned an RFC number and it is
important to submit each proposal regardless of the final status.

After you receive a final status from the Eng Council, do the following:

1. In your RFC file (`.md` extension), edit the assigned
   `name` for your RFC. For example, if you were assigned `RFC-9999`,
   the first line should look like the following:

   ```
   {% verbatim %}
   {% set rfcid = "RFC-9999" %}
   {% endverbatim %}
   ```

1. Rename your RFC file (`.md` extension) with the assigned RFC number. For example,
   if you were assigned `RFC-9999`, rename your file to `9999_<filename>.md`.

1. Edit the TOC entry for your RFC in
   [//docs/contribute/governance/rfcs/_toc.yaml][toc-file] to reflect the RFC number.
   For example, if you were assigned `RFC-9999`, your entry may look like
   the following:

   ```
   - title: "RFC-9999: My RFC proposal"
     path: /docs/contribute/governance/rfcs/9999_<filename>.md
   ```

1. Update the metadata for your RFC proposal. If you correctly filled out the
   initial metadata, you only need to update the following values in
   [docs/contribute/governance/rfcs/_rfcs.yaml][rfc-yaml].

   * `name`: Use the RFC name that was assigned. For example, `"'RFC-9999'`.
   * `file`: Use the filename that you used in the steps above. For example,
     `'9999_<filename>.md'`.
   * `status`: Use the status that was assigned to your RFC which can be
     `'Accepted'` or `'Rejected'`.
   * `reviewers`: Use the list of reviewers from your change. For example, if your
     reviewers were abarth@google.com, cpu@google.com, and vaas@google.com,
     `['abarth@google.com', 'cpu@google.com', 'vaas@google.com']`.
   * `Reviewed`: Use the date (year-month-day) on which your RFC was approved
     or rejected. For example, March 15th 2020 would be `'2020-03-15'`.

   The metadata for an accepted RFC proposal looks like the following:

   ```
   - name: 'RFC-0010'
     title: 'zx_channel_iovec_t support for zx_channel_write and zx_channel_call'
     short_description: 'This RFC introduces a new mode to zx_channel_write and zx_channel_call that copies input data from multiple memory regions rather than from a single contiguous buffer.'
     authors: ['bprosnitz@google.com']
     file: '0010_channel_iovec.md'
     area: ['Zircon']
     issue: ['60623']
     gerrit_change_id: ['433621']
     status: 'Accepted'
     reviewers: ['cpu@google.com', 'kulakowski@google.com', 'abarth@google.com', 'pascallouis@google.com']
     submitted: '2020-09-25'
     reviewed: '2020-10-31'
   ```

1. Upload a patch to your change with the updates to the RFC files.

   Note: For more information on creating a patch, see
   [Create and upload a patch](/docs/development/source_code/contribute_changes.md#create_and_upload_a_patch).

1. Once approved, work with your facilitator to submit your RFC. Specifically,
   your facilitator must +2 the CL.

You have successfully submitted an RFC proposal.

[rfc-overview]: /docs/contribute/governance/rfcs/README.md
[contribute changes]: /docs/development/source_code/contribute_changes.md
[rfc-template]: TEMPLATE.md
[eng-council]: ../eng_council.md
[toc-file]: https://ci.android.com/edit?repo=fuchsia/fuchsia/master&file=docs/contribute/governance/rfcs/_toc.yaml
[toc-ref]: /docs/contribute/docs/documentation-navigation-toc.md
[rfc-yaml]: https://ci.android.com/edit?repo=fuchsia/fuchsia/master&file=docs/contribute/governance/rfcs/_rfcs.yaml
