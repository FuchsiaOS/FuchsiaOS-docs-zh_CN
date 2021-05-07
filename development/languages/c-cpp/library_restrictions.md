# Library restrictions

## third_party/abseil-cpp

Decision: **do not use** `absl` in new code. Generally, `absl` is a
poor fit for use cases in Fuchsia. We will remove `absl` from the tree
once existing clients of it are migrated away. This removal work is tracked in
[fxbug.dev/59428](https://fxbug.dev/59428).

## third_party/googletest

Note: The googletest library includes both the former gtest and gmock
projects.

### Gtest

Use the Gtest framework for writing tests everywhere except the Zircon
directory. It provides the `TEST` and `TEST_F` macros as well as the `ASSERT`
and `EXPECT` variants we use.

Inside the Zircon directory, use `system/ulib/zxtest` instead. It provides a
Gtest-like interface with fewer dependencies on higher-level OS concepts like
mutexes (things we want to test). It also supports writing tests in
C which is required for some layers.

### Gmock

Gmock has several components. We allow the gmock matchers such as
`ElementsAre()`.

There are varying opinions on the team on the function mocking functions
(`MOCK_METHOD` and `EXPECT_CALL`).

Pros:

  * It can be very efficient to do certain types of mocking.
  * Some people feel that Gmock-generated mocks are easier to read than the
    equivalent custom code.
  * Lack of a mocking library means some people might not write good tests.

Cons:

  * Gmock provides a domain-specific language. Not everybody understands this
    language, and the complex use of templates and macros make it hard to
    diagnose problems.
  * Some aspects of Gmock encourage overly constrained mocks.
  * Combinations of the above can make it harder to make changes to mocked
    code later.

Decision: **do not use** the mocking functionality of gmock (`MOCK_METHOD` and
`EXPECT_CALL`).
