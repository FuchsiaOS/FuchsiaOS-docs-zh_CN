# Testing C/C++: Questions and Answers

You are encouraged to add your own questions (and answers) here!

[TOC]

## Q: Do we have Sanitizer support?

A: This is work in progress (SEC-27). ASAN is the closest to release (just
requires symbolization, TC-21).

## Q: How do I run with ASAN?

A: TBD

## Q: Do we have Fuzzers enabled?

A: No, sanitizer work takes precedence. Automated fuzz testing is SEC-44.

## Q: Do we use gmock?

A: Using the mocking functionality of gmock is
[disallowed](library_restrictions.md).
