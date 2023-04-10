A FIDL handle represents a unique capability in some system, usually the Zircon
kernel. *Handle rights* are a FIDL-legible enumeration of the privileges that
capability carries with it - for instance, whether or not the resource the handle
represents may be written to, inspected, signalled, and so on.

Rights are validated at encode and decode time, ensuring that a given handle
carries the set of privileges the interface author allotted for it.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_support_exports_tutorial.md>>
