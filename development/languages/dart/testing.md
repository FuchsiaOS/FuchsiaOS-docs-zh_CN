# Testing


The `dart_test` target is appropriate for unit tests.
Each target yields a test script in the output directory under:
```sh
out/<build-type>/gen/path/to/package/target_name.sh
```
This script simply runs the given tests in the Flutter shell *on the host*.

The `//scripts/run-dart-action.py` script may be used to run multiple test
suites at once:
```sh
scripts/run-dart-action.py test --out out/<build-type> --tree //topaz/shell/*
```
It also works with a single suite:
```sh
scripts/run-dart-action.py test --out out/<build-type> --tree //topaz/shell/armadillo:test
```

## Code coverage

To generate an HTML coverage report from all `dart_test`s, first build them
with `fx build` and then run:
```sh
scripts/dart/report_coverage.py --report-dir ...
```

This script runs all of the dart tests in your `<out>/host_tests/` dir with
coverage enabled. Under the hood, each test uses the coverage collection support
from [flutter](
https://github.com/flutter/flutter/wiki/Test-coverage-for-package:flutter).

The basic logic is:
```
for test in host_tests:
  covered_lines += lines in test's package that were covered by test
  total_lines += all lines in test's package
```

So if there are packages that have no tests at all, they won't be considered in
the denominator of the report, which can give you a misleadingly high coverage
number.
