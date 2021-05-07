# fconfig

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fconfig set-device <device-name> [options]
	set-device <device-name>  Sets the device properties supplied in options for the device matching the name.
	get [<device-name>.]<property-name>: Prints the value of the property or empty string if not found.
	get-all [<device-name>] prints all settings for the default device, or the device-name if provided
	   If there is no default device, an empty collection of settings is printed.
	list: Lists all settings.
	remove-device <device-name>  Removes the configuration for the given target device.
	Options:
	[--bucket <bucket>]  - specify the GCS bucket to be retrieve prebuilt images and packages.
	[--image <image>]    - specify the GCS image to be retrieve prebuilt images and packages.
	[--device-ip <addr>] - specify the IP address for the target device
	[--ssh-port <port>]  - specify the port number to use for SSH to the target device.
	[--package-repo <path>]  - override the default package repository path for the target device.
	[--package-port <port>]  - override the default port number use by the package server for this device.
	[--default]              - marks this device as the default device.
```

