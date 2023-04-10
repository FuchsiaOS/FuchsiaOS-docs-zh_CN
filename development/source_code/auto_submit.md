# Gerrit auto-submit

Fuchsia's Gerrit code review site supports an automatic change submission
feature. Any change that is opted in will automatically be submitted after being
approved and passing presubmit checks.

Note: Auto-submit is a Fuchsia-specific feature and its use and behavior do not
generalize to other Gerrit hosts, such as Chromium and Android, that use Commit
Queue or have their own auto-submit functionality.

## Usage

When adding reviewers in the Gerrit UI using the **REPLY** dialog, select the
**+1** for the **Fuchsia-Auto-Submit** label.

![demonstration of setting Fuchsia-Auto-Submit +1 in Gerrit](/development/source_code/auto_submit_usage.gif)

After your change meets all the submit requirements (generally a **Code-Review
+2** vote and owner approval of all affected files), the auto-submit bot will
apply the **Commit-Queue +2** label. Once all presubmit checks pass, your change
will automatically be submitted.

If you want your change to land as soon as possible after approval, it's
recommended that you set **Commit-Queue +1** before (or at the same time as)
sending your change for review. When auto-submit applies the **Commit-Queue +2**
label, it will skip rerunning any checks that have already passed within the
last 24 hours, so submission often doesn't need to wait for checks to rerun.

## FAQs

### How long does it take for auto-submit to submit my change? {#latency}

Auto-submit is implemented as a cron job that runs every 10 minutes, so it may
take up to 10 minutes for **Commit-Queue +2** to be applied to your change after
being approved.

### How do I tell if a change has auto-submit enabled?

If the author of a change has opted into auto-submit, a **Fuchsia-Auto-Submit
+1** tile will appear under **Trigger Votes** in the left column of the Gerrit
UI.

![Fuchsia-Auto-Submit +1 tile](/development/source_code/auto_submit_selected.png)

### I'm a reviewer on a change with auto-submit enabled. Can I approve it without submitting? {#unresolved-comments}

If you leave unresolved comments at the time you grant **Code-Review +2**, the
auto-submit bot will not submit the change until all comments are resolved.

However, the change author can still manually set **Commit-Queue +2** to submit
the change. If you think the change should not be submitted, then it's
recommended that either you withhold **Code-Review +2** or, if another reviewer
has already approved the change, set **Code-Review -2**.

