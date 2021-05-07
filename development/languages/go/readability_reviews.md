# Go readability reviews

Go readability reviews are an optional way to receive mentorship from
experienced Go programmers while working on Fuchsia.

## How to get a review

The suggested workflow:

1.  Author: Get a CR+2 as you normally would (from your teammates, OWNERS, etc).
2.  Author: If you'd like a go readability review, add
    go-readability-reviewers@fuchsia-infra.iam.gserviceaccount.com
3.  Author: At this point you can either merge the change (if you want to
    respond to comments in a follow-up change) or wait for the readability review
    (if you want to incorporate the comments in the current change).
4.  The GWSQ service will assign a readability reviewer.
5.  Reviewer: Take a look and leave comments. If there are no comments, please
    just CR+1. Please do not CR+2.
6.  If the change has already been merged, the author should address the
    comments in a new change that is reviewed by the readability reviewer.

{% dynamic if user.is_googler %}

## Volunteer to do reviews

You can ask to become a reviewer by emailing
fuchsia-go-readability-reviewers@google.com. As of writing, all reviewers have
Go readability in google3, but the group's owners may choose to admit members
who don't.

{% dynamic endif %}
