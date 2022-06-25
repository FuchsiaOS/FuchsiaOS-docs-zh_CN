# Code sample style guidelines {#overview}

This document describes how to incorportate code samples in documentation,
and specific style guidelines for code samples. This includes:

  *  [Code sample best practices](#code-sample-best-practices)
  *  [Code sample checklist](#code-sample-checklist)
  *  [Code sample style guide](#code-sample-guide)

For information on general documentation standards, including file types, locations, and general
tone, see the [Fuchsia documentation standards][doc-standard].
For specific guidance on word choice, style, and structure, see the
[Fuchsia documentation style guide][style-guide].


## Code sample best practices {#code-sample-best-practices}

When creating a code sample for a part of Fuchsia that you are deeply familiar with,
consider how a new user would read the sample and try to anticipate their needs.
Think about the process from end-to-end and include prerequisite steps to completing the process
and specify what success looks like.

For example, consider all of the prerequisite information needed before starting to use
the code sample. Make sure you're not overlooking information that is necessary to use the
sample but no longer present in your day-to-day workflow.

This might be because you have done these steps so many times that these steps
no longer stand out as necessary to the procedure. It also might be because this
prerequisite information only needed to be completed once and as a result, you
can't recall those steps at the time of writing documentation. If possible, try running
your sample from the very beginning, and verify that you have all of the prerequisite
information documented.

Likewise, it's important to let the user know when they have successfully completed
a given procedure correctly. To increase user confidence in your sample, make sure you
specify what the user's code should look like and how the user can confirm that they
have completed running your sample successfully.


## Code sample checklist {#code-sample-checklist}

If you are including a code sample in your documentation, review the following
list before submitting your contribution, to ensure code sample clarity:

*   **Include a "Prerequisites" section**, as the first section within
      your documentation.
      Having prerequisite information gathered within
      documentation, before the user starts the process, prevents the user
      from becoming unnecessarily blocked or frustrated.
      *   Prerequisite information can include any of the following:
          *   Editing environment variables.
          *   Running necessary scripts before starting the procedure.
          *   Obtaining device access.
          *   Including `BUILD` dependencies.
          *   Importing libraries.
*   **Link to existing documentation** where applicable.
      For example, a prerequisite for the process you are documenting
      might be a paved Fuchsia target device.
      Rather than restating how to pave one’s device, link to an existing
      “Pave” topic that already exists on [fuchsia.dev](https://fuchsia.dev/),
      such as [Build and pave quickstart](development/build/build_and_pave_quickstart.md).
*   **Avoid using `foo`, `bar`, or other vague placeholder names** if you are
    including placeholders in your code sample.
    Instead, use a name that expresses what that placeholder's function
    is within the code. For more information, see [Avoid vague placeholders](#avoid-vague-placeholders).
*   **Anchor developers within the process** by stating the obvious.
    Anticipate that someone might run code samples without thoroughly reading
    your documentation. As a result, make sure your codes samples are spatially
    aware. For more information, see [Specify placement](#specify-placement).
*   **End sections with a code sample summary** that details what the finished
    code is supposed to look like at a given point in a procedure.
    For more information, see [Specify placement](#specify-placement)
    and [Confirm success](#confirm-success).
*   **Describe the steps needed to test the process** and show what successful
    terminal output looks like.

## Code sample style guide {#code-sample-guide}

The following are actionable best practices for creating easily understandable
code samples in documentation.

### Avoid vague placeholders {#avoid-vague-placeholders}

Code sample placeholder names and values should represent their purposes within
the code, avoiding abstract placeholders like `foo` and `bar`.

* Use a placeholder name that expresses what the placeholder’s function is
  within that code.
  Doing so gives developers a real-world example that they can refer back
  to at a later point.

* Code samples should be able to be copied into the terminal and run
  successfully without extensive alteration by the user.

  Note: Developers might need to edit certain parts of the code sample to
  include user-specific values, like Fuchsia board types. If this is
  the case, include a self-explanatory placeholder within `<var></var>` tags,
  like <var>BOARD-TYPE</var>. Placeholders like this must also be wrapped
  within `<pre></pre>` tags.

Refer to the following example of avoiding vague placeholders.

#### Example

<section class="kd-tabbed-horz" id="placeholder-table">
  <article class="selected">
    <header id="not-recommended">Not Recommended</header>
<p>
To add a service, include the following:
  <pre><code class= "prettyprint">"services": [ "fuchsia.example.Foo" ],</code></pre>
</p>
  </article>
  <article>
    <header id="recommended">Recommended</header>
<p>
To add a service, you must edit your component manifest (.cmx).
For example, adding the <code>fuchsia.sys.Launcher</code> <code>service</code>
to your component manifest gives your component the ability to launch
other components.
</p>
<p>
  <pre class= "prettyprint">
  "sandbox": {
              "services": [ "fuchsia.sys.Launcher" ],
          }</pre>
</p>
</article>
</section>

### Specify placement {#specify-placement}

Code samples should be specify where that code should be
located within a given file.

For example, if a line of code should be located within a specific function,
then the code sample should demonstrate that spatial order, rather than
show that line of code without context.

Refer to the example of specifying code location.

#### Example

<section class="kd-tabbed-horz" id="specify-table">
  <article class="selected">
    <header id="not-recommended">Not Recommended</header>
<p>
<p>Add the following:</p>
  <pre class= "prettyprint">syslog::fx_log_info!("{}, log!", greeting());</code></pre>
</p>

  </article>
  <article>
    <header id="recommended">Recommended</header>
<p>
Include your log message within your source file, which in this
case is, <code>main.rs</code>:
</p>

  <p><pre class= "prettyprint">syslog::fx_log_info!("{}, log!", greeting());</code></pre></p>

<p>At this point, <code>main.rs</code> should look like this:</p>

  <p>
  <pre class= "prettyprint">
  use fuchsia_syslog as syslog;

  fn main() {
      syslog::init().expect("should not fail");
      syslog::fx_log_info!("{}, log!", greeting());
      println!("{}, world!", greeting());
  }
  …</pre>
  </p>
  </article>
</section>

### Confirm success {#confirm-success}

As a user, when you’re unfamiliar with a new process, it’s difficult to know if
you have completed that process _correctly_, even if you've
completed all of the documented steps.

Include a section in your how-to guide that specifies how developers can confirm that
they have successfully implemented a procedure. If possible, this section should
include the terminal output of the expected result. Doing so can help increase
user confidence.

Refer to the following example of confirming success in code samples.

#### Example

<section class="kd-tabbed-horz" id="success-table">
  <article class="selected">
    <header id="not-recommended">Not Recommended</header>
<p>
By adding the above code, you have enabled logging in Fuchsia.
</p>

  </article>
  <article>
    <header id="recommended">Recommended</header>
<p>
At this point you have enabled logging in Fuchsia.

To confirm that you have logging enabled in your component, complete the
following steps:
</p>
<ol>
  <li>Ensure that <code>fx serve</code> is running in a shell tab. If it is not, open a shell
      tab and run <code>fx serve</code>.</li>

        <p><pre><code class="devsite-terminal">cd ~/fuchsia</code></pre></p>

        <p><pre><code class="devsite-terminal">fx serve</code></pre></p>

  <li>In a new shell tab, navigate to your <code>fuchsia</code> directory and run <code>ffx log</code>.</li>

        <p><pre><code class="devsite-terminal">cd ~/fuchsia</code></pre></p>

        <p><pre><code class="devsite-terminal">ffx log</code></pre></p>

  <li>In a new shell tab, navigate to your fuchsia directory and run the
      <code>hello_world_rust</code> component:</li>

        <p><pre><code class="devsite-terminal">cd ~/fuchsia</code></pre></p>

        <p><pre><code class="devsite-terminal">ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm</code></pre></p>

  <li>Navigate to the shell tab where you ran <code>ffx log</code>.</li>

      <p>You should be able to see your logging text, which in this example
      is <code>Hello log!</code>.</p>
</ol>
  </article>
</section>

[doc-standard]: contribute/docs/documentation-standards.md
[style-guide]: contribute/docs/documentation-style-guide.md
[fuchsia]: https://fuchsia.dev/
