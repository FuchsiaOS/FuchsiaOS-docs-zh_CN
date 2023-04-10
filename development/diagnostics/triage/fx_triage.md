# Triage (fx triage)

Triage analyzes snapshots for out-of-range values. The analysis is specified by
configuration files.

To fetch and analyze a fresh snapshot from the default device:

```
fx triage
```

To analyze an existing snapshot:

```
fx triage --data /path/to/snapshot.zip
```

or

```
fx triage --data /path/to/unzipped_snapshot_dir
```

To specify config files to use:

```
fx triage --config path/file1 --config path/file2 --config "path/with/globs/*.triage"
```

Note that the `triage` command, not the OS, must expand the globs, so put the
path in quotes.

Config file format is described in [Configuring 'fx triage'](config.md). It
includes:

*   Selectors which specify the data to extract from the inspect.json produced
    by snapshot.zip.
*   Eval expressions which specify calculations.
*   Actions to take on specified values.
*   Tests to ensure your actions trigger (or not) appropriately with sample data
    you supply.
