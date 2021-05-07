# fssh

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fssh [options] [args]
  -device-ip string
    	Serves packages to a device with the given device ip address. Cannot be used with --device-name."
    			  If neither --device-name nor --device-ip are specified, the device-name configured using fconfig.sh is used.
  -device-name string
    	Serves packages to a device with the given device hostname. Cannot be used with --device-ip."
    			  If neither --device-name nor --device-ip are specified, the device-name configured using fconfig.sh is used.
  -help
    	Show the usage message
  -level value
    	Output verbosity, can be fatal, error, warning, info, debug or trace. (default info)
  -private-key string
    	Uses additional private key when using ssh to access the device.
  -sshconfig string
    	Use the specified sshconfig file instead of fssh's version.
  -verbose
    	Print informational messages.
```

