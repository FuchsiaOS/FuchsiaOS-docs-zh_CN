# API Documentation Readability Rubric

## Overview

This document contains guidance on writing API documentation for Fuchsia. It
applies both to public-facing APIs (those surfaced via an SDK) and
Fuchsia-internal ones. Public facing API documentation will be reviewed by the
[API Council](/docs/contribute/governance/api_council.md) for adherence to this
rubric.

## Overall commenting rules

In most cases, documentation should follow the language's style guide for
comments. If there is a rule in this document that contradicts the
language-specific rules, follow this document's guidance. In some cases, the
language-specific rules take precedence; these special cases are identified
below.

Here are the links to language-specific guidelines for languages likely to be
used in the Fuchsia repository: [C and
C++](/docs/development/languages/c-cpp/cpp-style.md),
[Dart](/docs/development/languages/dart/style.md)
[Rust](https://github.com/rust-lang-nursery/fmt-rfcs/blob/HEAD/guide/guide.md),
[Java](https://google.github.io/styleguide/javaguide.html),
[Kotlin](https://kotlinlang.org/docs/reference/coding-conventions.html#documentation-comments).
We also recommend reading [Google's guidelines on API
documentation](https://developers.google.com/style/api-reference-comments).

## Communicating with care

Fuchsia documentation is publically available, and should be
written in a technical and neutral tone. There are some explicit restrictions
on what you can write below, but they aren't intended to be comprehensive - use
good judgment!

 * Do not reference proprietary information. This includes personally
   sensitive information such as personally identifiable information and
   authentication keys.
 * Do not use swear words or other potentially aggressive language (words like,
   e.g., "stupid")

## General style

 * Fuchsia uses U.S. English, and follows the
   [Fuchsia document standards](/docs/contribute/docs/documentation-standards.md) and
   [style guide](/docs/contribute/docs/documentation-style-guide.md)
 * Do not list authors explicitly. Author information goes out of date quickly,
   as developers move to different projects. Consider providing a maintainers
   file, although be wary that this goes out of date, too.
 * Optimize your code for the intended display (e.g., use markdown or Javadoc as
   intended).
<!-- * Do not write TODO(username), write TODO(reference-to-bug). Bug ownership
   goes out of date quickly, as developers move to different projects. This
   includes documentation on unimplemented APIs and implementation notes. -->

Only apply the following rules in the absence of language-specific practices and
guidance:

 * Documentation should immediately precede the element it is documenting.
 * Use markdown for comments. The style of markdown is the style understood by
   the tool most likely to consume the API.
   * Use backticks for code blocks instead of 4-space indents.
 * All comments should use complete sentences.

## API elements

 * A **public facing API element** is one that is made available to developers
   via an SDK. All public facing API elements (including, but not limited to
   methods, classes, fields, types) must have a description. Internal libraries
   should be documented; there should be a good reason if they are not.

 * All parameters must have a description, unless that description would be
   redundant with the type and name.
   * If it is not obvious from the type what a parameter's legal values are,
     consider changing the type. For example, {-1, 0, 1} is less useful than an
     enum with {LESS\_THAN, EQUAL\_TO, GREATER\_THAN}.
   * Otherwise, document the behavior of the API for all possible input values.
     We discourage undocumented values.

 * All return values must have a description, unless that description would be
   redundant with the type and name.
   * If a method or function returns a subset of its return type, document the
     subset.
   * Document all returned errors and the circumstances under which they can be
     produced.
   * For example, if the method's return type is zx\_status\_t, and it only
     returns ZX\_OK and ZX\_ERR\_INVALID\_ARGS, your documentation must state
     that explicitly.
   * If it is not immediately obvious what a particular return value means, it
     must be documented. For example, if a method returns ZX\_OK, you don't
     need to document it. If a method returns the length of a string, it
     should be documented.

 * All possible thrown exceptions must have a description, which must include
   the conditions under which they are thrown, unless obvious from the type and
   name.
   * Some third party code does not document exceptions consistently. It may
     be hard (or impossible) to document the behavior of code that depends such
     APIs. Best effort is acceptable; we can resolve resulting issues as they
     arise.
   * Document whether exceptions are recoverable and, if so, how to recover
     from them.

 * For any API elements that are extensible, indicate whether they are intended
   to be extended, and requirements for those who might want to extend them.
   * If an API is extensible for internal reasons (e.g., testing), document
     that. For example, you should document if you have allowed a class to be
     extended in order to make it easy to create test doubles.

 * Document deprecated API elements.
   * Documentation on deprecated API elements must state what a user is expected
     to do instead of using the API.
   * Plans to eliminate the API should be clearly documented (if they exist).
   * If an explanation of the deprecation status of an API element would reduce
     the quality of the API documentation, consider providing a pointer to
     further information, including URLs and bug identifiers.

## API behavior

Document user-facing invariants, as well as pre- and post-conditions.

 * As a rule, ensure that there are assertions / tests to enforce these
   conditions.
 * Preconditions and postconditions that require explicit user action should
   be documented. For example, provide documentation if an `Init()` method
   needs to be called before anything else happens.
 * Correlations between parameters or return values (e.g., one has to be less
   than another) should be documented.

### Concurrency

Document the concurrency properties of APIs that have internal state.

 * FIDL servers may execute requests in an unpredictable order. Documentation
   should account for situations where this might affect the behavior the caller
   observes.
 * Every API with internal state falls into one of the following categories.
   Document which one, using the following terms:
   * **Thread-safe**: This means invocations of individual elements of the API
     (e.g., methods in a class) are atomic with respect to other concurrent
     processes. There is no need for a caller to use any external
     synchronization (e.g., a caller should not have to acquire a lock for the
     duration of the method invocation). You may still describe your API as
     thread-safe if a caller needs to use external synchronization to make
     references to instances of the API visible to other threads (e.g., by
     setting and getting a global pointer to an instance of a class with atomic
     operations).
   * **Thread-unsafe**: This means that all methods must use external
     synchronization to ensure invariants are maintained (e.g., mutual
     exclusion enforced by a lock).
   * **Thread-hostile**: This means that the API element should not be accessed
     from multiple threads (e.g., it has implementation details that rely on
     unsynchronized access to static data behind the scenes, like strtok()).
     This should include documentation about thread affinity (e.g., it uses
     TLS). It is only allowed in Fuchsia APIs by exception.
   * **Special**: This means that the correct concurrent use of this API
     requires thought, please read the docs. This is especially relevant when
     entities need to be initialized and references to them published in a
     specific way.
   * **Immutable**: The other four classes assume that internal state is
     mutable and thread safety is guaranteed by synchronization. Immutable
     classes appear constant without any additional synchronization, but you
     have to maintain strict rules about serialization / deserialization and
     how references to the object are shared between threads.
 * An API is **blocking** if it is not guaranteed to make progress. Document
   the blocking properties of your APIs.
   * If an API is blocking, the documentation must state what is required for
     the code to make progress, unless blocking is a low probability event that
     requires implementation understanding.
     * An example of when you must document a method's blocking behavior is when
       it blocks waiting for a response on a channel.
     * An example of when you do not have to document a method's blocking
       behavior is when it may block if lock starvation is a theoretical
       possibility under high load.
   * An API is not considered blocking only because it takes a long time to
     finish. A slow algorithm should not be documented to be blocking.
   * Documentation should only state that an API is non-blocking when the
     non-blocking behavior is critical to its use (for example, if an API
     returns a future).
 *  An API is **reentrant** if it may be safely interrupted in the middle of its
    execution and then called again. Document the reentrance properties of your
    APIs.
    * APIs may be assumed to be reentrant. Documentation must state if an API
      is not reentrant.
 * Document whether a function relies on **thread-local storage (TLS)** to
   maintain its invariants, and any preconditions and postconditions related to
   that TLS (e.g., if it needs to call an initializer once per thread).

### Ownership

Document ownership and liveness properties.

 * For parameters or return values that are stored beyond the life of a
   function, or resources allocated by the function and passed back to the
   caller, or resources with particular ownership constraints that must be
   observed by a set of APIs (i.e., shared resources), ownership and liveness
   must be documented.
 * Document who is responsible for releasing any associated resources.
 * Where appropriate, documentation should state the protocol for releasing
   those resources. This can be a special issue when memory allocation
   routines differ between the caller of an API and the API.
   * Languages should call out default ownership behavior in their style
     guides.

### Nullness

All parameters and return values must have their nullness properties defined (if
they are of a nullable type).

 * Even in Dart!
 * Where appropriate, refer to parameters and return values as **nullable** (may
   contain null) or **non-null** (may not contain null).

### Units

For all parameters and return types, units must be well defined (whether by
documentation or by type).

## Best practices

This section contains guidance that should be taken into consideration when
writing comments. It contains opinions, rather than the unambiguous rules given
above.

* A reader should not have to look at an API's implementation to figure out what
  it does. Consider writing documentation that would allow a reader to
  implement your API independently based on the documentation. If you need to provide
  additional details on how your API works, create and link to additional docucumentation
  on Fuchsia.dev.
* Avoid jargon that may not be obvious to the reader (think: "if I am
  interested in this API, will I definitely know what this word means?"). If
  jargon is Fuchsia-specific and not defined, add it to the [glossary](/docs/glossary.md).
* Avoid abbreviations and acronyms. When you need to use them, explain them.
  If the abbreviation is widely used in the industry (e.g., "Transmission
  Control Protocol / Internet Protocol" (TCP/IP)), you do not need to explain
  it, but you should consider giving a link for more context.
* Code samples should be considered whenever they might be useful. Providing
  an example can often make patterns clearer. We recommend an example of API
  for every top level API element (e.g., class).
* It should be clear how to use your API from the comments.
  * Consider writing examples as separate programs and linking to them, but be
    careful about stale links in docs.
  * Examples should all compile and run.
* When someone who has read the docs asks a question that should be answered by
  the docs, improve the docs.
* Always add value. Don't restate what is already indicated by the type signature.
  The Don't Repeat Yourself (DRY) principle applies. The following is
  not useful, because it repeats the same information twice:

``` java
 /**
  * Returns an instance of Foo.
  * @return an instance of Foo.
  */
 public Foo getFoo() { ... }
```

* Similarly, if the comment is very obvious, avoid making it. If, for example,
  a property is guaranteed by the type system, you do not need to document it
  separately. However, bear in mind that your API description should be enough
  to enable an independent implementation.
* Consider documenting performance considerations and resource consumption
  issues, but also remember that such issues are often implementation dependent
  and change over time, whereas the contract for your method will probably
  remain the same. Consider including this information in implementation notes
  / release notes instead.
* Avoid creating running words that are not compound words. For example "notready"
  is two words run together. Use an appropriate separator, for example "not ready",
  "notReady", "not_ready", or "not-ready").
* Avoid documenting features that may change rapidly over time,
  unless you specifically call out that feature may change over time. The more
  you prescribe, the less flexibility you give to future maintainers. However,
  recognize that it might not matter, since your users will depend on every
  behavior. See also [Hyrum's Law](http://www.hyrumslaw.com/).

