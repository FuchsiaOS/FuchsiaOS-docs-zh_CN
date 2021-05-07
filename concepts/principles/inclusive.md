# Inclusive

Fuchsia is an open source project that is inclusive by design,
from the architecture of the platform
to the open source community that we’re building.

Applying the principles of inclusion
through these dual lenses is a challenge we embrace.
We have not yet achieved all of our goals,
but we’re committed to doing the work to uphold this principle
with the help of our developer community.

## Fuchsia architecture is inclusive by design

### Developers can use their runtime and language of choice {#bring-your-own-runtime}

**[Fuchsia Interface Definition Language (FIDL)](/docs/concepts/fidl/overview.md)
allows diverse clients and services to interoperate**

Fuchsia is highly extensible:
developers can create components using the language and environment they prefer.
Both components and FIDL protocols are accessible to any runtime.
Software from different runtimes can integrate together to form a cohesive
experience. Fuchsia simplifies the development model,
making nearly all user space software a component,
from system services to end-user applications.

This principle is also known as Bring Your Own Runtime (BYOR).

### Fuchsia is designed to support a wide range of hardware

**[Fuchsia Driver Framework](/docs/concepts/drivers/fdf.md)
allows for a diverse hardware ecosystem**

Fuchsia aims to have a binary-stable interface for drivers.
In this approach,
developers can write drivers once and
these drivers will continue to work as Fuchsia evolves.
There’s no need to modify or recompile drivers when there’s an
update to Fuchsia. This allows for a large hardware ecosystem that
is scalable and easier to maintain.

### Anyone can build and test Fuchsia

**[Fuchsia's emulator (FEMU)](/docs/get-started/set_up_femu.md)
makes it easier for most development environments to run Fuchsia**

FEMU allows you to test Fuchsia components and applications
without needing a Fuchsia device. FEMU looks and behaves like a Fuchsia device,
with the exception that no paving is required.
FEMU simulates different processes and environments
that any developer can use to test and build Fuchsia.

## Open source community

### All developers are welcome to contribute

**[Guidelines and resources](/CONTRIBUTING.md)
are available to help Fuchsia developers**

Google and the Fuchsia team are committed
to preserving and fostering a diverse, inclusive, and welcoming community.
As an open source effort, we welcome high-quality, well-tested contributions
from all. [Our code of conduct](/CODE_OF_CONDUCT.md)
is in place to ensure that community discussions are productive and kind.

### Inclusive language is a core value

**[Respectful code practices](/docs/contribute/respectful_code.md)
reduce harm and bias**

Fuchsia's values include treating each other with dignity.
It’s important that everyone can contribute
without facing the harmful effects of bias and discrimination.
Our respectful code guidelines aim to eliminate terms
that perpetuate discrimination in our codebase, user
interfaces, and documentation.

### Communication channels are open

**[Our bug tracking system](/docs/contribute/report-issue.md)
and [mailing lists](/docs/contribute/community/get-involved.md)
are public**

The open source community can stay informed about Fuchsia updates and progress
by joining our mailing lists.
Fuchsia invites developers to contribute and report issues though our
bug tracking system.
The Fuchsia project uses Gerrit's web-based UI to manage code and
documentation reviews.

### Our roadmap is public

**Fuchsia is a [work in progress](/docs/contribute/roadmap/index.md)**

As the project evolves,
Fuchsia is striving to be as open as possible about the state of
the code and roadmap. The [Fuchsia RFC process](/docs/contribute/governance/rfcs/README.md)
aims to provide a consistent and transparent path
for making project-wide, technical decisions.
