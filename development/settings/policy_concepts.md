# Policy concepts

## What is a policy?

The Fuchsia Settings API protocols are fairly static and their behavior cannot
be customized. A few interfaces have parameters that can be configured at
build-time for a particular product, but none can be adjusted dynamically.

Fuchsia Settings provides this feature through the concept of a policy. A policy
modifies the behavior of Fuchsia Settings at runtime. Since the adjustments
happen at runtime, policies are modified through FIDL APIs.

The policy API is readable and writable by any client with the appropriate
policy permissions. The ability to control the policies is a separate permission
since a policy API client can affect the behavior for all Fuchsia Settings API
clients, for example by setting a minimum and maximum brightness value to the
same value.

## Anatomy of a policy API

Policies are defined in terms of **transforms**, which modify the requests sent
to the Fuchsia Settings API. An example of a transform would be a minimum
brightness limit.

Brightness is a simple setting and there’s only one brightness value to be
controlled, but suppose a device had multiple displays, each with their own
brightness value. In that case, a client of the policy API would need to specify
which display that their minimum brightness limit applies to. The policy API
calls this identifier a **target**, specifying an aspect of a setting.

## Policy API example {#policy-example-table}

```fidl
library fuchsia.settings.policy;

alias PolicyId = uint32;

@discoverable
protocol VolumePolicyController {
    GetProperties() -> (struct {
        properties vector<Property>:MAX;
    });

    AddPolicy(struct {
        target Target;
        parameters PolicyParameters;
    }) -> (struct {
        policy_id PolicyId;
    }) error Error;

    RemovePolicy(struct {
        policy_id PolicyId;
    }) -> () error Error;
};

type Property = table {
    1: target Target;
    2: available_transforms vector<Transform>:MAX;
    3: active_policies vector<Policy>:MAX;
};

type Target = strict union {
    1: stream fuchsia.media.AudioRenderUsage;
};

type Transform = strict enum : uint8 {
    MAX = 1;
    MIN = 2;
};

type Policy = table {
    1: policy_id PolicyId;
    2: parameters PolicyParameters;
};

type PolicyParameters = strict union {
    1: min Volume;
    2: max Volume;
};

type Volume = table {
    1: volume float32;
};
```

Note: For the full FIDL definition, see [volume_policy.fidl][volume_policy_fidl].

The policy API looks similar to the Fuchsia Settings API but there are some
notable differences.

### Add and remove instead of Set

Instead of a single Set method, the policy API offers the pair of `AddPolicy`
and `RemovePolicy`. The Fuchsia Settings API only offers a `Set`, since settings
values are always expected to exist. For example, there is always a default
brightness level even if no client ever set it. A client would never need to
“remove” the brightness level, only modify it. In contrast, a policy transform
is an action to be taken on an input to the Fuchsia Settings API, so there can
be any number of policies active for a particular target, including none.

### Get instead of Watch

The Fuchsia Settings API return values through Watch methods that implement the
[hanging get pattern][hanging-get]. Policy only offers a simple `GetProperties`
method to provide information about policy targets, including possible
transforms and active policies. Policy API clients are not expected to respond
to changing policies so a hanging get isn't necessary. Clients should only need
to know the active policies before attempting to add or remove a policy.

Each policy has a unique identifier that is generated when added, that can be
used to remove it later. To add a policy, clients need to provide the target the
policy is aimed at, as well as their parameters for the policy.

### Different namespace

The policy API lives in the `fuchsia.settings.policy` namespace instead of the
`fuchsia.settings` namespace that the Fuchsia Settings API lives under. Using a
different namespace requires a different permission for clients so that being
able to use the Fuchsia Settings API doesn't automatically grant permission to
use the policy API.

In the [policy interface for volume example](#policy-example-table), any client
can apply a maximum volume limit to any target. This allows multiple policies to
apply to the same target, either from the same client or from multiple clients.
All policies added are considered valid unless there’s a logical error (such as
a minimum limit that’s higher than the maximum limit).

The policy API will attempt to respect all active policies and apply the
intersection of policies. In this case, the strictest maximum volume limit will
be the lowest maximum volume among the active policies. In the event that the
current lowest maximum volume limit is removed, the next lowest policy will take
effect.

## Policy concept definitions

The following is a recap of policy API concepts:

* Target: Unique identifier for a controllable aspect of a setting.

  For example, the targets in the volume policy are the individual audio streams

* Transform: An operation on the values for a setting target.

  Policy clients can specify customizable parameters for transforms, ex. max
  volume limit

* Policy: A client-specified transform, along with its parameters.

  Policies each have a globally unique ID to identify them

* Property: A collection of information about a policy target.

  A property contains information about what transforms are possible on a given
  target, and what policies are active

<!-- link labels -->
[volume_policy_fidl]: /sdk/fidl/fuchsia.settings.policy/volume_policy.fidl
[hanging-get]: /docs/development/api/fidl.md#hanging-get
