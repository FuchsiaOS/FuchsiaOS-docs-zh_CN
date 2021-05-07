# fserve

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fserve [options]
  -bucket string
    	Specify the GCS bucket for the prebuilt packages.
  -device-ip string
    	Serves packages to a device with the given device ip address. Cannot be used with --device-name."
    		  If neither --device-name nor --device-ip are specified, the device-name configured using fconfig.sh is used.
  -device-name string
    	Serves packages to a device with the given device hostname. Cannot be used with --device-ip."
    		  If neither --device-name nor --device-ip are specified, the device-name configured using fconfig.sh is used.
  -help
    	Show the usage message
  -image string
    	Specify the GCS file name for prebuild packages.
  -kill
    	Kills any existing package manager server.
  -level value
    	Output verbosity, can be fatal, error, warning, info, debug or trace. (default info)
  -name string
    	Name is used as the update channel identifier, as reported by fuchsia.update.channel.Provider. (default "devhost")
  -prepare
    	Downloads any dependencies but does not start the package server.
  -private-key string
    	Uses additional private key when using ssh to access the device.
  -repo-dir string
    	Specify the path to the package repository.
  -server-port string
    	Port number to use when serving the packages.
  -sshconfig string
    	Use the specified sshconfig file instead of fssh's version.
  -version string
    	SDK Version to use for prebuilt packages.
```

