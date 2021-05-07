# Integrating the Integrator Development Kit (IDK)

Integrating the IDK is the process of consuming the IDK and turning
it into an SDK that is specific to a development environment so it can be used
directly by developers.

The main entry point for the ingestion process is a file at
`//meta/manifest.json`.
As with every metadata file in the IDK, the manifest follows the JSON schema
at `//meta/schemas/manifest.json`.

This file contains a list of all the elements included in this IDK, represented
by the path to their respective metadata file.
Each element file is guaranteed to contain a top-level `type` attribute, which
may be used to apply different treatments to different element types. For example,
generating a build file for a FIDL library or just moving a host tool to a
convenient location in the final development environment.

The existence of the various metadata files as well as the exhaustiveness of
their contents should make it so that the ingestion process may be fully
automated.
JSON schemas may even be used to generate code representing the metadata
containers and let the ingestion program handle idiomatic data structures
instead of raw JSON representations.

The metadata schemas will evolve over time.
In order to allow consumers of that metadata to adjust to schema changes, the
main metadata file contains a property named `schema_version`, which is an opaque
version identifier for these schemas.
This version identifier will be modified every time the metadata schemas evolve
in a way that requires the attention of a developer.
IDK consumers may record the version identifier of the metadata they used to last
ingest an IDK and compare that version identifier to next IDK's version
identifier in order to detect when developer action may be required.


