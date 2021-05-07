# fvdl

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fvdl [--sdk] <command> [<args>]
entry point for fvdl Commands to start/stop the emulator via fuchsia virtual device launcher (VDL)
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --sdk             running in fuchsia sdk (not inside the fuchsia code
                    repository)
  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  start             Starting Fuchsia Emulator
  kill              Killing Fuchsia Emulator
```

## kill

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fvdl kill [-d <vdl-path>] [--launched-proto <launched-proto>]
Killing Fuchsia Emulator
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -d, --vdl-path    device_launcher binary location. Defaults to looking in
                    prebuilt/vdl/device_launcher
  --launched-proto  required, file containing device_launcher process artifact
                    location.
  --help            display usage information
```

## start

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: fvdl start [-H] [-N] [--host-gpu] [--software-gpu] [--hidpi-scaling] [-u <upscript>] [--packages-to-serve <packages-to-serve>] [-p <pointing-device>] [-w <window-width>] [-h <window-height>] [-s <image-size>] [-f <device-proto>] [-e <aemu-path>] [--aemu-version <aemu-version>] [-d <vdl-path>] [--vdl-version <vdl-version>] [-x <grpcwebproxy>] [-X <grpcwebproxy-path>] [--grpcwebproxy-version <grpcwebproxy-version>] [-v <sdk-version>] [--gcs-bucket <gcs-bucket>] [--image-name <image-name>] [-l <emulator-log>] [--port-map <port-map>] [--vdl-output <vdl-output>] [--nointeractive]
Starting Fuchsia Emulator
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -H, --headless    bool, run emulator in headless mode.
  -N, --tuntap      bool, run emulator with emulated nic via tun/tap.
  --host-gpu        bool, run emulator with host GPU acceleration, this doesn't
                    work on remote-desktop with --headless.
  --software-gpu    bool, run emulator without host GPU acceleration, default.
  --hidpi-scaling   bool, enable pixel scaling on HiDPI devices.
  -u, --upscript    path to tun/tap upscript, this script will be executed
                    before booting up FEMU.
  --packages-to-serve
                    comma separated string of fuchsia package urls, extra
                    packages to serve after starting FEMU.
  -p, --pointing-device
                    set pointing device used on emulator: mouse or touch screen.
                    Allowed values are "touch", "mouse". Default is "touch".
  -w, --window-width
                    emulator window width. Default to 1280.
  -h, --window-height
                    emulator window height. Default to 800.
  -s, --image-size  extends storage size to <size> bytes. Default is "2G".
  -f, --device-proto
                    path to fuchsia virtual device configuration, if not
                    specified a generic one will be generated.
  -e, --aemu-path   path to aemu location. When running in fuchsia repo,
                    defaults to looking in prebuilt/third_party/aemu/PLATFORM.
                    When running in fuchsia sdk, defaults to looking in
                    $HOME/.fuchsia/femu.
  --aemu-version    label used to download AEMU from CIPD. Default is
                    "integration". Download only happens if aemu binary cannot
                    be found from known paths.
  -d, --vdl-path    device_launcher binary location. When running in fuchsia
                    repo, defaults to looking in prebuilt/vdl/device_launcher.
                    When running in fuchsia sdk, defaults to looking in
                    directory containing `fvdl`.
  --vdl-version     label used to download vdl from CIPD. Default is "latest".
                    Download only happens if vdl (device_launcher) binary cannot
                    be found from known paths.
  -x, --grpcwebproxy
                    enable WebRTC HTTP service on port, if set to 0 a random
                    port will be picked
  -X, --grpcwebproxy-path
                    location of grpcwebproxy, When running in fuchsia repo,
                    defaults to looking in prebuilt/third_party/grpcwebproxy
                    When running in fuchsia sdk, defaults to looking in
                    $HOME/.fuchsia/femu.
  --grpcwebproxy-version
                    label used to download grpcwebproxy from CIPD. Default is
                    "latest". Download only happens if --grpcwebproxy is set and
                    grpcwebproxy binary cannot be found from known paths or path
                    specified by --grpcwebproxy_path.
  -v, --sdk-version fuchsia sdk ID used to fetch from gcs, if specified, the
                    emulator will launch with fuchsia sdk files fetched from
                    gcs. To find the latest version run `gsutil cat
                    gs://fuchsia/development/LATEST_LINUX`.
  --gcs-bucket      gcs bucket name. Default is "fuchsia".
  --image-name      image file name used to fetch from gcs. Default is
                    "qemu-x64". To view availabe image names run `gsutil ls -l
                    gs://fuchsia/development/$(gsutil cat
                    gs://fuchsia/development/LATEST_LINUX)/images`.
  -l, --emulator-log
                    file path to store emulator log. Default is a temp file that
                    is deleted after `fvdl` exits.
  --port-map        host port mapping for user-networking mode. This flag will
                    be ignored if --tuntap is used. If not specified, an ssh
                    port on host will be randomly picked and forwarded. ex:
                    hostfwd=tcp::<host_port>-:<guest_port>,hostfwd=tcp::<host_port>-:<guest_port>
  --vdl-output      file destination to write `device_launcher` output. Required
                    for --nointeractive mode. Default is a temp file that is
                    deleted after `fvdl` exits. Specify this flag if you plan to
                    use the `kill` subcommand.
  --nointeractive   bool, turn off interactive mode. if turned off, fvdl will
                    not land user in ssh console. A ssh port will still be
                    forwarded. User needs to specify --vdl-output flag with this
                    mode, and manually call the `kill` subcommand to perform
                    clean shutdown.
  --help            display usage information
```

