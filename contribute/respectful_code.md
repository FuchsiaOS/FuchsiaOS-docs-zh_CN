# Respectful Code

Inclusivity is central to Fuchsia's culture, and our values include treating
each other with respect and dignity. Everyone should be able to contribute 
to Fuchsia without facing the harmful effects of bias and discrimination. 
This respectful code policy provides guidance to address language that can
perpetuate discrimination or harm in the codebase, UIs, and documentation.

## Policy

Terminology that is derogatory, hurtful, or perpetuates discrimination, either
directly or indirectly, should be avoided and will be replaced.

## What is in scope for this policy?

Anything that a contributor would read while working on Fuchsia, including:

- Names of variables, types, functions, files, build rules, binaries, exported
  variables...
- Test data
- System output and displays
- Documentation (both inside and outside of source files)
- Commit messages

## Principles

- Be respectful: Avoid bias and harm. Derogatory, ableist, or unnecessarily
  gendered language are not useful to describe how things work.
- Respect culturally sensitive language: Some words may carry significant
  historical or political meanings. Be mindful of this and use
  alternatives.

## How do I know if particular terminology is OK or not?

Apply the principles above. If you have any questions, you can reach out to
fuchsia-community-managers@google.com.

## What are examples of terminology to be avoided?

These lists are NOT meant to be comprehensive. They contain common examples
found in documentation. If you see disrespectful language, report it.

**Specific terms**

| Term        | Suggested alternatives                                        |
| ----------  | ------------------------------------------------------------- |
| master      | primary, controller, leader, host                             |
| slave       | replica, subordinate, secondary, follower, device, peripheral |
| whitelist   | allowlist, exception list, inclusion list                     |
| blacklist   | denylist, blocklist, exclusion list                           |
| insane      | unexpected, catastrophic, incoherent                          |
| sane        | expected, appropriate, sensible, valid                        |
| sanity check| check                                                         |
| crazy       | unexpected, catastrophic, incoherent                          |
| redline     | priority line, limit, soft limit                              |
| white glove | top tier service; meticulous, thorough support                |
| blackout    | blocked out                                                   |
| build cop   | build gardener                                                |
| dummy       | placeholder                                                   |

**Idioms**

Use descriptive and factual statements instead of idioms. Idioms can suffer
from the same problems described above, and also they can be difficult to
understand for people with a different cultural context than you.

* For example, instead of "this is not black or white," use "this is more
  nuanced."
* Instead of "this is the blind leading the blind," explain what you mean, like
  "the reference you point to is inaccurate because ..."

## What if I encounter terminology that violates this policy?

When implementing code, differing from the language in the specification 
may interfere with the ability to understand the implementation. In these 
circumstances, we suggest one of the following, in order of decreasing
preference:

1. If using alternate terminology doesn't interfere with understanding, use
   alternate terminology.
2. Failing that, do not propagate the terminology beyond the layer of code that
   is performing the interfacing. Where necessary, use alternate terminology
   at the API boundaries.
