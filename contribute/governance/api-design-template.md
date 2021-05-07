# Fuchsia API Design Document template

This template details the sections that should be included in your API design
document as well as the questions that should be answered by your API Design
Document.

## Summary

A one paragraph summary of your change to the Fuchsia API.

## Goals and use cases

Your API design document is expected to answer the following questions
regarding your API's use cases:

  + What problem does this API or API feature solve?

  + What would users of your API be able to accomplish?

This section acknowledges that there is more than one solution that could
resolve the problems your API is intended to fix. Construct your "Use cases"
section in a way that doesn’t presuppose that the design proposed by your
document is the only correct way to solve those use cases.

## Design

This section contains the technical details of your API design.

This section contains the following:

  + A high-level description of your approach, including:
    + The key choices that you’ve made in your design.
    + The actual code definition of the API, such as the FIDL definition of an
      interface.

  + A Gerrit change link that contains the code for your API:

    + Your API design should conform to the [API readability rubric](/docs/concepts/api/README.md)
      for the language that you use to write the API. The interface should be
      fully documented at this stage but it does not need to be implemented.

  + An explanation of the choices behind your API design and why you’ve
    made those design choices.
  + An explanation of how your API might evolve in the future.

## Unknowns

This section answers the following questions regarding
your design's assumptions:

  + What information are you missing that might help improve the design?
  + Are there areas of your design that could be improved?
  + Which parts of your design are you most interested in receiving feedback on
    and why?

## Usability

This section answers the following questions regarding the usability of your API:

  + Are the semantics of your API intuitive from its signature(s)?
  + Have you designed the appropriate extensions points to allow for the future
    evolution of your API?
  + Does your API behave similarly to other Fuchsia APIs that do similar things?
  + How does your API behave compared to similar APIs for other platforms?

A good framework for thinking through the usability of your API is to
write example programs that use your API. That exercise gets you thinking about
how users experience your API and lets you experience any potential
drawbacks of your design.

If you find your API difficult to use while writing these examples, consider
revising your API to improve its usability. Your users are end-developers.
They should be key stakeholders when you consider how to design your API.

## Testing

This section answers the following questions regarding
your API's approach to testing:

  + How do you plan to test your API?
    + You might have unit tests for your implementation, but you
      might also want a medium-size test that exercises your implementation
      through your API.
      + Consider using `lib/component/cpp/testing`.
  + If developers were to rely on your API feature, how would they test their
    code?
    + Consider providing a mock implementation of your API that clients
      can use for testing purposes.

## Performance considerations

There is often a tension between performance and usability. The performance
considerations for an API often vary by the frequency with which the API is
used. This section should describe the choices that you’ve made to balance these
concerns. Consider consulting the [API readability rubric](/docs/concepts/api/README.md)
for language-specific guidance about how to balance these concerns.

This section answers the following questions regarding how
your API design affects performance:

  + Does your API involve a large number of round-trips across a
    process or thread boundary?
  + Does your API involve blocking on a remote process or thread?
  + Does your API involve copying large amounts of data?
  + How many queries per second (QPS) do you expect your API to receive?
  + How much data do you expect a typical query to transport?

## Security considerations

This section answers the following questions regarding
how your API design considers security:

  + Does your API expose security-sensitive information?
  + Does your API let its users manipulate security sensitive resources?
  + Are the users of your API isolated from each other?
  + Does your API respect an object-capability discipline?
  + Does your API encourage your users to use your API securely?
    + Does your API encourage time-of-check to time-of-use (TOCTOU)
      vulnerabilities?
    + Does your API clearly separate any control planes from any data planes?

If your API has non-trivial security considerations, you should consult
with the security team and go through a formal security review. If this is the
case, contact the [API council](https://groups.google.com/a/fuchsia.dev/g/api-council) about requesting a security review.

When a security review is performed, provide a link to your security
review in this section.

## Privacy considerations

This section answers the following questions regarding
how your API design considers privacy:

  + Does your API expose privacy-sensitive information?
  + Does your API involve any personally identifiable information?
  + Does your API involve any device identifiers?
  + Does your API provide users control over how information is shared?

If your API has non-trivial privacy considerations, go through a formal privacy
review. When a privacy review is performed, provide a link to your privacy
review in this section.

## Drawbacks and alternatives

Your API design document is expected to answer the following questions
regarding how you've considered drawbacks as well as alternative
implementations:

  + Are there any disadvantages to your API design?
  + What alternative designs did you consider?
    + Why aren't you using these alternatives?
    + Are there trade-offs or scenarios where these alternative designs may
      be appropriate?

## Submit your API Design Document

To submit your API Design Document, do the following:

1. Duplicate this markdown file.
2. Edit the contents of that duplicate markdown file to include the answers to the template.
3. Save your markdown file with a hyphenated name of your choosing.
4. Submit your API Design Document markdown file by following the steps [Contribute changes](/docs/development/source_code/contribute_changes.md).

After your Design Document has been submitted, it is reviewed by the API
Council. For more information on the API Design Document review process, see the
[Fuchsia API Council Charter](/docs/contribute/governance/api_council.md).
