# ACPI debugging

## ACPICA debug interfaces

To turn on ACPICA's debug output, pass the "enable_acpi_debug = true" build
argument to GN.  When this option is enabled, ACPICA uses two global variables
to control debug output.

### AcpiDbgLevel

AcpiDbgLevel is a bitmap of values defined in
third\_party/lib/acpica/source/include/acpica/acoutput.h with the prefix
"ACPI\_LV\_".  For convenience, there are some pre-defined verbosity levels:
ACPI\_LV\_VERBOSITY1, ACPI\_LV\_VERBOSITY2, ACPI\_LV\_VERBOSITY3.  These control
types of tracing events to log.  For example, if you want to trace all function
calls and mutex operations, you can set AcpiDbgLevel to

"ACPI\_LV\_FUNCTIONS | ACPI\_LV\_MUTEX"

### AcpiDbgLayer

AcpiDbgLayer is a bitmap of values defined in
third\_party/lib/acpica/source/include/acpica/acoutput.h.  These do not have a
common prefix, but are listed as "Component IDs".  These control which
submodules of ACPICA are to be traced.  For example, to trace through the
namespace logic and the executor, you can set AcpiDbgLayer to

"ACPI\_NAMESPACE | ACPI\_EXECUTOR"

### Setting these values

One easy place to set these in the AcpiOsInitialize method that we define in
third\_party/lib/acpica/source/os\_specific/service\_layers/osfuchsia.cpp.
One technique that may be useful is zeroing both values in AcpiOsInitialize, and
setting it to a non-zero value immediate before a call into ACPICA of interest.

### AcpiDebugTrace

There is additionally a method named AcpiDebugTrace in the ACPIA API.  It
supposedly supports tracing particular ACPI methods by their 4-character
namespace names (but with no scoping to particular Nodes).  See the ACPICA
manual for details.
