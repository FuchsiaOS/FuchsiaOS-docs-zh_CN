# Host Side Fidl

This document is a short summary of what's available now for host side FIDL, and what may be available in the future.

## What’s Available?

Encoding and decoding of structs and tables that contain no zircon handles in C++ only.

* Use of handles (or consequently FIDL interface requests and the like) will cause the host side libraries to fail.
* In the future this will be verified via a mechanism like NoHandles.

## What’s not Available?

Any use of interfaces.

* Trying to use a FIDL file that mentions an interface will cause the host side runtime to fail to compile.
* In the future some verification mechanism will be available here too.

## What is out of scope?

Emulation of arbitrary zircon handles (particularly VMO’s).

## What is possibly in scope?

Interfaces communicating over a socket transport (implies not exchanging handles).

## What is undecided?

Emulation of channels on host side (maybe via overnet).
