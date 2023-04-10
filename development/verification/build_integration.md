# Build-Integrated Verification Procedures

Several verification procedures are integrated into the Fuchsia build. All of these procedures
execute some form of check against build output artifacts. The purpose of this integration is to
catch potential issues as early as possible in the development process.

## Verifying system-wide properties with scrutiny

A number of verification procedures are realized by invoking [scrutiny via ffx][ffx-scrutiny].
These procedures are codified in gn templates in the [build/security/verifier][scrutiny-verifiers]
directory. Examples include verifying well-formed capability routes over the component topology and
verifying the set of static packages pre-installed on the system.

In general, these verification procedures analyze whole collections of files to test for particular
properties and/or ensure that these collections match an allowlist of manually vetted artifacts.

## Verifying configuration files

**Note*: _This section applies to Googlers only._ Verifying structured configuration files is
currently enabled only for non-eng builds of Google products.

In some cases system components are designed to support different configurations for eng and non-eng
[build types][build-types]. When components are engineered using this strategy, it is important to
protect the integrity of non-eng builds by verifying the correct configuration on such builds.

### Verifying structured configuration files

To verify non-eng Google product builds for your structured configuration, you must take the
following steps:

1. Identify safe configuration values expected in non-eng builds;
1. File a bug to track landing your component or feature safely;
1. Add configuration values to policy files, mark them as transitional;
1. Check in new component or feature;
1. Clean up policy files and bugs.

Each step of this process is detailed below.

#### 1. Identify safe configuration values expected in non-eng builds

Structured configuration files will be verified by comparing the values generate by the build
against a golden file that denotes safe configuration values to ship to users. Start by writing
down what these values are for your component.

#### 2. File a bug to track landing your component or feature safely

If you don't already have a bug tracking the component or feature you're working on, be sure to file
one and mention verified structured configuration for non-eng builds. You will refer to this bug
in subsequent steps of this process.

#### 3. Add configuration values to policy files, mark them as transitional

You will need to modify all files in
`//vendor/google/security/policy/**/*structured_config_policy*.json5`. These files contain a set of
structured configuration values grouped by components identified by the Fuchsia URLs (see examples
below). Any components or configuration values marked as transitional are only checked if they are
found in the build, and are otherwise ignored. Components or values that are _not_ marked as
transitional are required: non-transitional components must be present in the build output and
each non-transitional configuration values must match the expected value denoted in the policy
file.

For example, when adding a **new component** the policy file might look something like this:

```json5
{
    components: {
        // ...existing components...
        // new component:
        // <High-level description of how to configure new component>
        "fuchsia-pkg://fuchsia.com/new-frobinator#meta/new-frobinator.cm": {
            // TODO(fxbug.dev/####): Remove transitional once `new-frobinator` is integrated into
            // the build.
            transitional: true,
            fields: {
                // <Precise description of why `frobinator_debugging_enabled=false` is a safe
                // configuration to ship to users>
                frobinator_debugging_enabled: false,
            },
        },
    },
}
```

If you were adding a **new field** to an **existing component** the policy file might look
something like this:

```json5
{
    components: {
        // ...existing components...
        // component with new fields:
        "fuchsia-pkg://fuchsia.com/existing-frobinator#meta/existing-frobinator.cm": {
            fields: {
                // ...existing fields appear as `<field_name>: <expected_value>`; e.g.,
                frobinator_internal_policy: "production",
                // <Precise description of why `frobinator_debugging_enabled=false` is a safe
                // configuration to ship to users>
                frobinator_debugging_enabled: {
                    expected_value: false,
                    // TODO(fxbug.dev/####): Remove transitional once `frobinator_debugging_enabled`
                    // is integrated into the build.
                    transitional: true,
                },
            },
        },
    },
}
```

The policy must include all structured configuration fields that may vary between eng and non-eng
builds. Be sure not to skip any of the `<Description>` or `TODO(fxbug.dev/####)` comments outlined
above.

#### 4. Check in new component or feature

It is now safe to check in the new component or feature and wire it up to the build and the fuchsia
system's component tree. If you make a mistake configuring the component on non-eng Google product
builds, this will show up as a build error during structured configuration policy checks.

#### 5. Clean up policy files and bugs

Edit the policy files described in Step 3 again, removing the `transitional` markings that were
added in Step 3.

Returning to the examples from Step 3, for the **new component** example, the final policy file
looks something like:


```json5
{
    components: {
        // ...existing components...
        // new component:
        // <High-level description of how to configure new component>
        "fuchsia-pkg://fuchsia.com/new-frobinator#meta/new-frobinator.cm": {
            fields: {
                // <Precise description of why `frobinator_debugging_enabled=false` is a safe
                // configuration to ship to users>
                frobinator_debugging_enabled: false,
            },
        },
    },
}
```

And the final policy file for a **new field** for an **existing component** looks something like:

```json5
{
    components: {
        // ...existing components...
        // component with new fields:
        "fuchsia-pkg://fuchsia.com/existing-frobinator#meta/existing-frobinator.cm": {
            fields: {
                // ...existing fields appear as `<field_name>: <expected_value>`; e.g.,
                frobinator_internal_policy: "production",
                // <Precise description of why `frobinator_debugging_enabled=false` is a safe
                // configuration to ship to users>
                frobinator_debugging_enabled:  false,
            },
        },
    },
}
```

Once these changes land, assuming work on the new component or feature is complete, it is safe to
close the bug mentioned in Step 2.

<!-- TODO(fxbug.dev/104819): Link to fxbug.dev page when better documentation is available.  -->
[build-types]: /docs/contribute/governance/rfcs/0115_build_types.md
[ffx-scrutiny]: https://fuchsia.dev/reference/tools/sdk/ffx#scrutiny
[scrutiny-verifiers]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/security/verifier/
