# Create SSH keys for Fuchsia devices

To make an SSH connection to a Fuchsia device, some `ffx` commands
(such as [`ffx target show`][ffx-target-show] and [`ffx log`][ffx-log])
require Fuchsia-specific SSH keys to be present on the host machine.

## Concepts

During development, one or more public SSH keys
(listed in `fuchsia_authorized_keys`) are loaded to a Fuchsia device,
typically when a new Fuchsia image is [flashed to the device][flash-device].
Once the device is loaded with these public SSH keys, the `ffx` commands
(running from the host machine where a matching private SSH key is stored)
can establish an SSH connection to the device.

By default, Fuchsia-specific SSH keys are stored in the
`$HOME/.ssh` directory of the host machine, as shown below:

```none {:.devsite-disable-click-to-copy}
$HOME/.ssh/fuchsia_ed25519
$HOME/.ssh/fuchsia_ed25519.pub
$HOME/.ssh/fuchsia_authorized_keys
```

Note: These keys are not password protected. Don't use these keys for
non-development devices.

These files contain the following:

*   `fuchsia_ed25519`: A private SSH key. The content of this file must not
    be revealed or shared.
*   `fuchsia_ed25519.pub`: A public SSH key that pairs with
    the private SSH key in `fuchsia_ed25519`.
*   `fuchsia_authorized_keys`: A list of one or more authorized public SSH keys.

The `fuchsia_authorized_keys` file must include the public SSH key in
`fuchsia_ed25519.pub`. During the flashing process, the `fuchsia_authorized_keys`
file gets uploaded from the host machine to the Fuchsia device.

If you have multiple development machines,
it's recommended that the Fuchsia SSH keys are synchronized across
your development machines. This may require you to copy the existing
Fuchsia SSH keys files from one machine to another.

## Generate SSH keys

To generate Fuchsia-specific SSH keys on your host machine, do the following;

Note: These Fuchsia-specific SSH keys are only used to connect to Fuchsia devices
during development. Generating these SSH keys won't alter your current SSH settings.

1.  Generate a new private and public SSH key pair:

    ```posix-terminal
    ssh-keygen -P "" -t ed25519 -f "${HOME}/.ssh/fuchsia_ed25519" -C "${USER}@$(hostname -f) Shared SSH Key for Fuchsia"
    ```

1.  Generate a `fuchsia_authorized_keys` file:

    ```posix-terminal
    ssh-keygen -y -f "${HOME}/.ssh/fuchsia_ed25519" > "${HOME}/.ssh/fuchsia_authorized_keys"
    ```

1.  Verify that Fuchsia-specific SSH keys are generated:

    ```posix-terminal
    ls ~/.ssh | grep fuchsia
    ```

    This command prints output similar to the following:

    ```none {:.devsite-disable-click-to-copy}
    $ ls ~/.ssh | grep fuchsia
    fuchsia_authorized_keys
    fuchsia_ed25519
    fuchsia_ed25519.pub
    ```

After creating new SSH keys, you'd need to
[flash your Fuchsia device][flash-device] again so that the device is loaded with
the new `fuchsia_authorized_keys` file.

<!-- Reference links -->

[ffx-target-show]: https://fuchsia.dev/reference/tools/sdk/ffx?skip_cache=true#show_8
[ffx-log]: https://fuchsia.dev/reference/tools/sdk/ffx?skip_cache=true#log_2
[flash-device]: ./flash-a-device.md
