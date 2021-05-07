# README.fuchsia File Syntax

`README.fuchsia` files are used to annotate third-party source libraries
with some useful metadata, such as code origin, version and license.

The format of these files consists of one or more directive lines,
followed by unstructured description and notes.

Directives consist of a directive keyword at the beginning of the line,
immediately followed by a colon and a value that extends to the end of
the line. The value may have surrounding whitespace, and blank lines may
appear before or between directives.

Several directives are described below, but other directives may appear
in `README.fuchsia` files and software that consumes them should not
treat the appearance of an unknown directive as an error. Similarly,
such software should match directive keywords case-insensitively.

Description lines are optional and follow a `Description` directive
that must appear on a line by itself prior to any unstructured
description text.

## Syntax

```
file                  := directive-line* description?
directive-line        := directive | blank-line
directive             := keyword ":" SPACE* value SPACE* EOL
value                 := NONBLANK ANYCHAR*
description           := description-directive description-line*
description-directive := "Description:" SPACE* EOL
description-line      := ANYCHAR* EOL
keyword               := [A-Za-z0-9][A-Za-z0-9 ]*
blank-line            := SPACE* EOL
SPACE                 := any whitespace character
EOL                   := end of line character
NONBLANK              := any non-whitespace, non-EOL character
ANYCHAR               := any character but EOL
```

## Common directive keywords

Common directive keywords include:

* `Name`

  Descriptive name of the component. This should be included if the name
  is not obvious from context.

  ```
  Name: OpenSSH
  ```

* `URL`

  The URL where the component lives. If the component is based on a
  specific release, then list that explicitly.

  ```
  URL: https://ftp.openbsd.org/pub/OpenBSD/OpenSSH/openssh-7.6.tar.gz
  ```

  Otherwise, list the vendor's website.

  ```
  URL: https://www.openssh.com/
  ```

  This directive may be repeated to include multiple URLs if necessary.

* `Version`

  Lists a version number or commit identifier for the software. If the
  version is apparent from the *URL* or commit history, then this may be
  omitted.

  ```
  Version: 7.6
  ```

* `License`

  The license under which the component is distributed. Only standard forms
  are accepted, e.g. MIT/X11, BSD, Apache 2.0.

  ```
  License: BSD
  ```

* `License File`

  File that contains a copy of the component's license. This must name
  an existing file in the repository, relative to the `README.fuchsia`
  file.

  ```
  License File: LICENCE
  ```

  This directive may be repeated to include multiple files if necessary.

* `Upstream Git`

  Links to the upstream Git repository from which this component has
  been branched. This should be included for any software branched from
  an external Git repository.

  ```
  Upstream Git: https://github.com/openssh/openssh-portable
  ```

* `Description`

  Marks the end of directives and the beginning of unstructured
  description, it must appear on a line by itself.

  ```
  Description:

  A short description of what the package is and is used for.
  ```

* `Local Modifications`

  Enumerate any changes that have been made locally to the package from the
  shipping version listed above.

  ```
  Local Modifications:

  Added README.fuchsia.
  Ported build rules from CMake to GN.
  ```
