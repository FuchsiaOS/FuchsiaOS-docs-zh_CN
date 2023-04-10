<!--

// LINT.IfChange

-->

# lights-cli

Get information about lights and control their brightness and color.

## Usage {#usage}

```none
lights-cli <device_path> print <id>
lights-cli <device_path> set <id> <brightness>
lights-cli <device_path> set <id> <red> <green> <blue>
lights-cli <device_path> summary
lights-cli list
```

## Commands {#commands}

### print {#print}

```none
lights-cli <device_path> print <id>
```

View the brightness and color (if applicable) of a light. The reported values
are floating point numbers between `0.0` (completely off) and `1.0` (completely
on).

### set {#set}

```none
lights-cli <device_path> set <id> <brightness>
```

Set the brightness of a light. For lights that support pulse-width modulation
`<brightness>` can be any number between `0.0` (completely off) and `1.0`
(completely on). For lights that only support simple on and off states
`<brightness>` should only be `0.0` (off) or `1.0` (on).

```none
lights-cli <device_path> set <id> <red> <green> <blue>
```

Set the color of a light. `<red>` `<green>` and `<blue>` can be any number
between `0.0` and `1.0` to control the intensity of each color component.

### summary {#summary}

```none
lights-cli <device_path> summary
```

View the total light count as well as the brightness and capabilities of each
light. Currently supported capabilities are `Brightness`, `Rgb`, and `Simple`.
`Brightness` is a value between `0.0` and `1.0` as explained in the `set`
command's description. `Rgb` is the RGB value of the light. `Simple` indicates
whether the light supports pulse-width modulation or only simple on and off
states.

### list {#list}

```none
lights-cli list
```

List the device paths of all lights.

## Examples

All examples for node "123" (to check available nodes type `ls /dev/class/light`).

### View the brightness of a light

```none {:.devsite-disable-click-to-copy}
$ lights-cli /dev/class/light/123 print AMBER_LED
Value of AMBER_LED: Brightness 1.000000
```
### View the brightness and color of a light

```none {:.devsite-disable-click-to-copy}
$ lights-cli /dev/class/light/123 print 1
Value of lp50xx-led-1: Brightness 0.745098 RGB 0.235294 0.176471 0.164706
```

### Set the brightness of a light

```none {:.devsite-disable-click-to-copy}
$ lights-cli /dev/class/light/123 set AMBER_LED 0.5
# This command exits silently.
```

### Set a light to display the color purple

```none {:.devsite-disable-click-to-copy}
$ lights-cli /dev/class/light/123 set 5 0.5 0 0.5
# This command exits silently.
```

### View the total light count and each light's brightness and capabilities

```none {:.devsite-disable-click-to-copy}
$ lights-cli /dev/class/light/123 summary
Total 1 lights
Value of AMBER_LED: Brightness 0.500000
    Capabilities: Brightness
```

### List the device paths of all lights

```none {:.devsite-disable-click-to-copy}
$ lights-cli list
/dev/class/light/123
```

## Notes

<<./_access.md>>

### Source code

Source code for `lights-cli`: [`//src/ui/light/bin/lights-cli/`][src]

[src]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/light/bin/lights-cli/

<!--

// LINT.ThenChange(//src/ui/light/bin/lights-cli/main.cc)

-->
