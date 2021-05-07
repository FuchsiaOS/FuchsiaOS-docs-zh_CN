  To get the server component up and running, there are three targets that are
  defined:

  * The raw executable file for the server that is built to run on Fuchsia.
  * A component that is set up to simply run the server executable,
    which is described using the component's manifest file.
  * The component is then put into a package, which is the unit of software
    distribution on Fuchsia. In this case, the package just contains a
    single component.

  For more details on packages, components, and how to build them, refer to
  the [Building components](/docs/development/components/build.md) page.
