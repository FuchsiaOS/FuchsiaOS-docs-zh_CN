# Defining Fuchsia package information with metadata files

## Introduction

Fuchsia's Gerrit installation has support for custom structured metadata files.
These files are designed to be:

* _human-readable_ and provide useful information such as the name, description
  and canonical URL of a package
* _machine-readable_ by having the files be defined using a standard format, and
  so can be used by automated systems for things like automatically adding
  reviewers to CLs or find bug components that alerting systems should file
  issues against

Currently, the only automated use for metadata files is for automatically adding
reviewers to CLs and emailing notifications when CLs are added/submitted. These
are described in the [Presubmits](#presubmits) section.

Metadata files only apply to first-party code.

## Format

Metadata files are:

* named `METADATA.textproto`
* specified using [Protocol Buffers version
  3](https://protobuf.dev/programming-guides/proto3/)
* written using the [Protocol Buffer Text Format Language
  Specification](https://protobuf.dev/reference/protobuf/textformat-spec/)

The metadata message types are as follows:

```proto
syntax = "proto3";

// This proto is located at
// https://github.com/googleapis/googleapis/blob/master/google/type/date.proto
import "google/type/date.proto";

// Monorail contains the project configuration for the Monorail issue tracker.
message Monorail {
  string project = 1;
  string component = 2;
}

// IssueTracker contains the project configuration for the Google
// Issue Tracker service.
message IssueTracker {
  int64 component_id = 1;
}

message Tracker {
  repeated Monorail monorail = 1;
  repeated IssueTracker issue_tracker = 2;
  // for_automation defines whether this tracker should be used by automated
  // processes (e.g. alerts).
  bool for_automation = 3;
}

message Presubmit {
  repeated string review_notify = 1;
  repeated string auto_reviewers = 2;
}

message Metadata {
  // name is the name of the API/Package/Program.
  string name = 1;
  string description = 2;
  // url points to some documentation/landing page.
  string url = 3;
  // Date this metadata was last reviewed by a human.
  google.type.Date last_reviewed_date = 4;
  repeated Tracker trackers = 5;
  // presubmits are used for defining presubmits.
  // The field is marked repeated for future expansion, but only
  // one message should be used.
  repeated Presubmit presubmits = 6;
}
```

Here is a fake example of a metadata file:

```
name: "Fuchsia source code automation"
description: "A binary for automating source code gardening tasks"
url: "https://fuchsia.dev"
last_reviewed_date: {
    year: 2022
    month: 1
    day: 23
}
trackers: {
    monorail: {
        project: "fuchsia"
        component: "EngProd"
    }
}

presubmits: {
  auto_reviewers: "frodo@example.com"

  review_notify: "sauron@example.com"
  review_notify: "gandalf@example.com"
}
```

## Multiple metadata files

To support large repositories that contain multiple distinct projects, multiple
`METADATA.textproto` files may be used throughout the file tree. Metadata files
apply to their own directory and all the directories below.

## Presubmits

The `presubmit` field allows the specification of reviewers who should be
automatically added to each changelist as a reviewer (`auto_reviewers`) and
those who should be notified via email when a changelist has been
uploaded/submitted (`review_notify`).

Here is an example hierarchy which contains multiple metadata files:

```
├── alice
│   ├── METADATA.textproto
│   └── README.md
├── foo
│   └── bar
│       ├── baz
│       │   └── METADATA.textproto
│       │   └── server.go
│       └── METADATA.textproto
│       └── hello.rs
├── METADATA.textproto
└── README.md
```

Reviewers specified in `/METADATA.textproto` are added/notified whenever there
is a change to any file in the entire repository. Reviewers in
`/foo/bar/METADATA.textproto` apply to anything in the `/foo/bar` directory, as
well as the child directory `/foo/bar/baz`. Reviewers in
`/foo/bar/baz/METADATA.textproto` only apply to files edited in the
`/foo/bar/baz/` directory.

## Relationship to other files

Metadata files have no relationship to other files, such as [`OWNERS`](owners.md) (used to specify the owners of directories) or [`README.fuchsia`](third-party-metadata.md) (used for defining metadata for third-party code).
