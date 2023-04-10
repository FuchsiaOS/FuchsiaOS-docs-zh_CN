# README

The purpose of this directory is to enable a cataloging of all `fidlc` errors.
Each markdown file in this directory represents the description for a unique
`fidlc` error.

## Adding a new error

To add a new error, use the following script:

```
cd ~/fuchsia
tools/fidl/scripts/add_errcat_entry.py {ERROR_NUMBER}\
  --bad={NUMBER_OF_BAD_EXAMPLES}\
  --good={NUMBER_OF_GOOD_EXAMPLES}
```

Replacing the values in brackets as necessary. For example, to create error
1000, with 2 good examples and 2 bad examples:

```
tools/fidl/scripts/add_errcat_entry.py 1000\
  --bad=2\
  --good=2
```

After running this script, you'll need to update the created files removing any
TODOs.

### Manually adding a new error

BIG NOTICE

Unless you have a good reason (you probably don't), you should add a new error
with the script above. This is mostly here for documentation's sake for the
event that the python script above isn't maintained.

END BIG NOTICE

To manually add a new error:

1. Create a new markdown file `_fi-{ERROR_NUMBER}.md` in
  `//reference/fidl/language/error-catalog/`. Replace `{ERROR_NUMBER}` with
  the error code. Pad the number with leading 0's. Copy the contents of an
  existing error so the content is consistent.

    > Error numbers must be monotonically increasing, dense, and never re-used.

1. Add the new error to the end of `//reference/fidl/language/errcat.md` by
  adding the following:

    ```
    <<error-catalog/_fi-{ERROR_NUMBER}.md>>
    ```

1. Create at least 1 good example in `//tools/fidl/fidlc/tests/fidl/good`.
    + If you're creating exactly 1 good example, the file name should be:
        + `fi-{ERROR_NUMBER}.test.fidl`
    + If you're creating more than 1 good example, the file names should be:
        + `fi-{ERROR_NUMBER}-a.test.fidl`
        + `fi-{ERROR_NUMBER}-b.test.fidl`
        + and so on...

1. Add a new entry to the end of `//error/_redirects.yaml` in the following format:

    ```
    - from: /fuchsia-src/error/fi-{ERROR_NUMBER}
      to: /fuchsia-src/reference/fidl/language/errcat.md#fi-{ERROR_NUMBER}
    ```

1. Create at least 1 bad example in `//tools/fidl/fidlc/tests/fidl/bad`.
    + If you're creating exactly 1 good example, the file name should be:
        + `fi-NNNN.test.fidl`
    + If you're creating more than 1 good example, the file names should be:
        + `fi-NNNN-a.test.fidl`
        + `fi-NNNN-b.test.fidl`
        + and so on...

1. Add a test case to `//tools/fidl/fidlc/tests/errcat_good_tests.cc` using the
    "good" example.

1. Add or update an existing test case using the "bad" example. A good way to
    find existing test cases is to grep for the error name. If there are no
    existing test cases, add a new one somewhere in `//tools/fidl/fidlc/tests`.

## Retiring an error

To retire an error:

1. Update existing errcat files to account for the newly retired error.
    1. Delete the good and bad examples:

        ```
        rm //tools/fidl/fidlc/tests/fidl/good/fi-{ERROR_NUMBER}.test.fidl
        rm //tools/fidl/fidlc/tests/fidl/bad/fi-{ERROR_NUMBER}.test.fidl
        ```

        Don't forget that sometimes there are `-a`, `-b`, etc FIDL files as
        well.  Check for these.
  1. Update `//reference/fidl/language/error-catalog/_fi-{ERROR_NUMBER}.md`
  to the following:

        ```
        ## fi-ERROR_NUMBER {:#fi-ERROR_NUMBER .hide-from-toc}

        Deprecated: This error code has been retired.
        ```

1. Update necessary C++ code

    + Update the existing error entry in
    `//tools/fidl/fidlc/include/fidl/diagnostics.h` from using the `ErrorDef`
    macro to the `RetiredDef` macro. For example, for error 10, this would look
    like the following:

        ```
        - constexpr ErrorDef<10, std::string_view> ErrInvalidIdentifier("invalid identifier '{}'");
        + constexpr RetiredDef<10> ErrInvalidIdentifier("error id fi-0010 has been retired.");
        ```
