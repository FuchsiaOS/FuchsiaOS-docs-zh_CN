# Library restrictions

## third_party/absl-cpp

Decision: **do not use** `<absl/synchronization/*>`. On Fuchsia, these classes
bottom out in `pthread_mutex_t` and `pthread_cond_t`, which are not the most
efficient primitives on Fuchsia. When `ABSL_INTERNAL_USE_NONPROD_MUTEX` is
defined, these primitives bottom out in something much more sophisticated.
Instead, please use `<lib/sync/*.h>`, which bottoms out in optimal
synchronization primitives on Fuchsia.

## third_party/googletest

*** aside
Note that the googletest library includes both the former gtest and gmock
projects.
***

Decision: **do not use** the mocking functionality of gmock (`MOCK_METHOD` and
`EXPECT_CALL`). It is allowed to use gmock matchers (such as `ElementsAre()`).
