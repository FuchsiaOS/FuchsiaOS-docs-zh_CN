# Shortlink rubric

Shortlinks (fuchsia.dev/go) are intended to be durable, have a meaningful
name, and be of general interest. The links are durable and use a
flat namespace, so it is desirable to carefully consider each proposal
to limit the number of created short links.

Reviewers should consider the following factors to approve pending short
link requests:

* Will the proposed link be relevant for a long time?
  Links are intended to be durable, so you should be reasonably confident
  that a link will continue to be of general interest for a long time.

  * <span class="compare-better">Positive example</span>:
    A general Fuchsia FAQ page or the current list of supported hardware. Even
    though they will change over time, the topics will be relevant
    indefinitely.

  * <span class="compare-worse">Negative example</span>:
    An RFC proposal that is under consideration. While the proposal is
    currently relevant, it will be either accepted or rejected. If accepted,
    it may be a suitable short link topic for reference.

* Does the link cover a general topic or concept? Is it relevant for a wide
  audience?

  * <span class="compare-better">Positive example</span>:

      * An overview of FIDL. This is a fundamental technology used by Fuchsia.
      * The Fuchsia security model. This is of interest to both users and
      developers.

  * <span class="compare-worse">Negative example</span>:

      * Documentation for a single system call. A link to the list of system
        calls would be more relevant.
      * Instructions for booting a specific model of NUC over Ethernet. A link
        to a more general page on booting Fuchsia on categories of devices
        would be of interest to a wider audience.

* Does the link duplicate an existing easy-to-use link that is unlikely to
  change?

  * <span class="compare-better">Positive example</span>:
    <fuchsia.dev/fuchsia-src/get-started> is unlikely to change, so
    <fuchsia.dev/go/get-started> is probably redundant.

* Is the link name meaningful, and does it comply with the
  [Respectful Code policy](/contribute/respectful_code.md)?
  Short links should be, of course, short, but not at the expense of being
  understood.

  * <span class="compare-better">Positive example</span>:
    Both of these examples are short and descriptive:

      * `/go/faq` (renders as fuchsia.dev/go/faq).
      * `/go/hardware-specs` (renders as fuchsia.dev/go/hardware-specs).

* Does the `to` link point to a document inside of the `/docs/` directory?

  * <span class="compare-better">Positive example</span>:

      * `to: /concepts/software_model.md`.

  * <span class="compare-worse">Negative example</span>:

      * `to: http://google.com/`.