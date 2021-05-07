# Test Environments

The build system is tightly coupled with how the continuous integration
infrastructure (from here on referred to as just "infra") discovers, aggregates,
and runs tests. The high-level idea is that the test authors specify how they
want their tests to run in GN and this information is propagated from the build
system to the infra.

More specifically

*   The build produces metadata files for each test specifying the required
    execution environments.
*   Infra groups the tests into shards that share the same environment.
*   For each shard, infra schedules a bot to run those tests.
*   Results from all shards are aggregated and reported.

## Environments

The specification of a test's `environments` in GN is what determines where and
how a test is run. It is given as list of scopes of the following form:

```gn
environments = [
  {
    dimensions = {
       <dimension key> = <value>
       ...
    }
    tags = ["<environment tags...>"]
    netboot = <boolean>
  },
  ...
]
```

See
[guest_integration_tests](/src/virtualization/tests)
for an example - and below for definitions of 'dimension' and 'tags'

### Default Behavior

If no environments are specified for a test, than default behavior is as
follows:

1.  `__is_fuchsia__`: test only runs in a QEMU instance running Fuchsia
1.  `__is_linux__`: test only runs on a Linux machine
1.  `__is_mac__`: test only runs on a Mac machine

(1) means that hardware is opt-in; test authors must explicitly specify hardware
environments in order to run tests there. The reasoning for this is that not all
tests need to run on hardware, test authors know best whether that is the case,
and that hardware is a scarce resource.

### Predefined environments

One may import
[//build/testing/environments.gni](/build/testing/environments.gni)
and use the environment-related convenience variables defined within. For
example, `basic_envs` includes all of the environments that are available to
anyone without special consultation with the infra team.

### Dimensions

`dimensions` here refer to
[Swarming](https://chromium.googlesource.com/infra/luci/luci-py/+/HEAD/appengine/swarming/doc/)
dimensions, where Swarming is the task distribution system used by the infra. A
dimension is effectively a key-value pair that describes a bot property that can
be targeted.

### Tags

Tags are arbitrary strings that may be attached to an environment. Setting
it amounts to removing the corresponding test from the normal testing pipeline;
in order then for that test to run, infra support for a new builder (to run
tests for particular tags) must be added. Labels are used for special tests
that require different configurations. Before using tags, please consult with
fuchsia-infra-team@google.com"

### Netboot

Netboot specifies whether to netboot instead of paving before running the tests
in the shard for that environment. If omitted, it will be treated as false.

## Validation

The `test_plaforms` list in
[//build/testing/platforms.gni](/build/testing/platforms.gni)
is the source of truth for what platforms are available for testing and what
dimensions they possess to match against. Say an environment *matches* a
platform entry if the former's `dimensions` is a subscope of the latter; say an
environment is *valid* for the current architecture if it matches a
`test_platforms` entry that doesn't specify a `cpu` value different than
`current_cpu`.

Environment validation happens at `gn gen`-time and can be summed up as

*   Each environment must be valid for some architecture.

*   Each test must have an environment that is valid for the given architecture.

### Example

Suppose platforms.gni consisted of

```gn
test_platforms = [
  {  # P1
    device_type = "QEMU"
    cpu = "x64"
  },
  { # P2
    device_type = "QEMU"
    cpu = "arm64"
  },
  { # P3
    device_type = "Intel NUC Kit NUC7i5DNHE"
    cpu = "x64"
  },
]
```

and consider the specification of

```gn
environments = [
  { # E1
     dimensions = {
       device_type = "Intel NUC Kit NUC7i5DNHE"
     }
  },
  { # E2
     dimensions = {
       device_type = "QEMU"
     }
  },
]
```

When `current_cpu` is x64, E1 and E2 are both valid and match against P1 and P3
respectively: the test is scheduled to run on a NUC and in QEMU. When
`current_cpu` is arm64, E1 is invalid but is ignored as E2 is valid and matches
P2: the test is scheduled to run in QEMU alone.
