Connecting to FIDL protocols within components is a combination of capability
routing and directory serving. This means that diagnosing connection issues can
cover a few different layers:

*   Client requests the protocol capability in its manifest.
*   Provider exposes the protocol capability in its manifest.
*   Component topology routes the capability from the provider to the client.
*   Provider is serving the protocol on the correct handle.
*   Client is attempting to connect to the correct protocol handle.

In this section, you'll explore some APIs and tools to help you find and fix
problems with component connections, and monitor the long-term health of your
components.
