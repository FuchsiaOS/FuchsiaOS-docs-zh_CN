<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0134" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC removes time synchronization as a dependency of software updates.

## Motivation

Fuchsia devices report a fixed "backstop" time until they synchronize time from
a network or real time clock. The time reported is currently implemented as
reporting the approximate time of the base image build, which could be a few
months old.

Unlike other early-boot users of time, the system update checks don't wait for
time synchronization. When time is not synchronized during an update check, the
package resolver may indirectly use the backstop time as part of TLS certificate
validation. This has the following issues:

 1. When the backstop time is older than the creation date of the certificate,
    the update fails with a [`CertNotValidYet`] error. Because the update and
    package servers may be accessed via different names, this failure can occur
    in multiple components. In this case, if time synchronization is broken,
    the device will be stuck and never update again.
 2. Expired certificates that were valid during the backstop time can be used
    to serve update responses and packages.

Removing time synchronization as a dependency of the software update process
solves the first issues, and makes update more reliable.

## Design

This RFC proposes that software updates should not rely on accurate time being
available.

Which means that it will:

* Allow certificates to have a validity period that is in the future.

* Always use backstop time to validate certificate, even if time has been
  synchronized.

## Implementation

A custom TLS certificates verifier needs to be implemented, and used in
`omaha-client` and `pkg-resolver`.

During construction, the verifier will get the backstop time from
[`ClockDetails`] of the UTC clock object, and keep it as a field of the struct.

The verifier will implement [`ServerCertVerifier`] trait using
[`WebPKIVerifier`] with a time function that returns the backstop time, if
[`WebPKIVerifier`] returns [`CertNotValidYet`] error, create another
[`WebPKIVerifier`] using a time that's within the validity period of the
certificate and call it again.

## Performance

The custom verifier simply wraps [`WebPKIVerifier`] for the happy path, and will
only call [`WebPKIVerifier`] twice if the first call returns [`CertNotValidYet`]
error.

## Ergonomics

N/A

## Backwards Compatibility

No changes to the API.

## Security considerations

This design will accept these certificates as valid:

* an expired TLS certificate that's newer than the backstop time
* a TLS certificate that's in the future

While this is not ideal, the first case can already happen today so it's not a
regression, and the second case has very little risk.

In addition, even if the device were tricked into installing a malicious update,
we still have verified execution, so it won't boot and will revert back. We also
have roll back protection, so a signed old build won't work either.

Keep in mind that an attacker wielding a compromised certificate and DNS control
could trick the time system into accepting an arbitrary time, so even if we make
update checks wait for time synchronization, it would not help with this
situation.

Certificate revocation is out of scope of this RFC, but would be a good area of
exploration in another one, although such a solution could require listing
expired certificates as revoked indefinitely.

## Privacy considerations

No impact on privacy.

## Testing

Unit tests the custom certificates verifier extensively, make sure that it will
only accept a certificate in the future if everything else is valid.

Integration test for `omaha-client` and `pkg-resolver` to verify that update
works with very old time.

## Documentation

N/A

## Drawbacks, alternatives, and unknowns

### Alternative: Wait for Time Synchronization

In this model, we will not update until the time has been synchronized, while
this gives us some security benefit, it's not much because time synchronization
could be compromised as well, however this model makes update depends on time
synchronization, which might be broken for various reasons, making updates less
reliable.

### Alternative: Wait for Time, but Update After Deadline

In this model, we'd wait some arbitrary amount of time for time to synchronize,
and perform an update check after some period if time has not yet synchronized.

While this solves the primary drawback of the previous alternative, it is
sensitive in other ways. The delay is arbitrary: a device caught in a boot loop
with a lower period than this delay would also never update.

Additionally, we still rely on backstop time, and this likely implies the need
to relax our certificate validation strictness.

## Prior art and references

ChromeOS does not have this problem, because it does not have backstop time and
there are no Google TLS certificates that were valid in 1970.


[`ServerCertVerifier`]: https://docs.rs/rustls/0.19.1/rustls/trait.ServerCertVerifier.html
[`WebPKIVerifier`]: https://docs.rs/rustls/0.19.1/rustls/struct.WebPKIVerifier.html
[`CertNotValidYet`]: https://docs.rs/webpki/0.22.0/webpki/enum.Error.html#variant.CertNotValidYet
[`ClockDetails`]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_zircon/struct.ClockDetails.html