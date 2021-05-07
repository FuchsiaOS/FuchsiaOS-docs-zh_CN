# SSH

SSH is the supported protocol for communication between a Fuchsia target device
and a host device.
This document describes how to properly set up an SSH connection between these
devices.

## Prerequisites

On the host side, a proper SSH distribution is required.

A public/private keypair is also needed.
It may be generated via the `ssh-keygen` command, or extracted from the running
SSH agent via `ssh-add -L`.

## Provisioning a device

There are two options for installing the public key onto the target.

### By installing it during paving (preferred)

Follow the instruction for [paving](bootserver.md) the target device, and add an
extra argument to the `bootserver` call pointing to the public key:
```
$ bootserver --authorized-keys $PUBLIC_KEY <other args>
```

### By modifying the Fuchsia image directly

The `fuchsia.zbi` image may be modified to include the public key using the
`zbi` tool:
```
$ zbi -o $FUCHSIA_DOT_ZBI -e data/ssh/authorized_keys=$PUBLIC_KEY
```

Note that this method is mainly designed for situations where paving is not
necessarily an efficient option (e.g. testing on an emulator).
Use with care.

## Connecting to a device

Provided that the address of the target device is known as `$TARGET_ADDRESS`,
open a shell on that device with:
```
$ ssh -i $PRIVATE_KEY fuchsia@$TARGET_ADDRESS
```

Note that if you got the key from your SSH agent, or if the key is in a well
known location (`$SSH_HOME`) under a well known name (`id_*`), you may omit the
`-i` argument.

Note also that the host keys for a Fuchsia target device are generated at first
boot, meaning that every time the device gets paved the keys are going to
change.
You may want to disable host key checking when connecting to a Fuchsia device to
avoid running into errors by adding the following flags:
```
-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
```
