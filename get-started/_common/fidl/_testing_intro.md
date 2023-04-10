[Integration testing](https://en.wikipedia.org/wiki/Integration_testing){:.external}
focuses on validating your component's behavior as it interacts with other
components on the system. Because of this, integration tests are typically built
separately from the main component and may declare the component under test
and other dependencies as children. Depending on the nature of the test,
dependency components may be provided as mocks or stubs to promote that the test
cases remain hermetic.
