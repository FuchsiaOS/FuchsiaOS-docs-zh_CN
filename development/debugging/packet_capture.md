# Packet Capture on Fuchsia

Packet capture is a fundamental tool for developing, debugging, and testing networking.

`fx sniff` is a development host command that:

* Runs the packet capture on the Fuchsia **target** device.
* Stores the packets in PCAPNG format on the Fuchsia development **host**.
* Streams out to a graphical user interface such as `Wireshark`.

`netdump` is a packet capturer with rich capture filter support. `fx sniff` internally invokes `netdump` with predefined capture filters that are necessary for Fuchsia's developer workflow. For use cases where `fx sniff` is not viable (e.g. when you have serial console access but without dev host connected), use `netdump` directly.

## Prepare the image {#prepare-image}

`netdump` is part of the [universe dependency list](/docs/concepts/build_system/boards_and_products.md#universe) of the core product. If a package server is available, there is no extra step to prepare the image. Just running `netdump` will fetch the binary.

If the package server is not available, make sure to bundle `netdump` into your set of base packages.

```shell
$ fx set core.x64 --with-base //src/connectivity/network/netdump
$ fx build
```


## How-to (On Host)

### Capture packets over WLAN interface

```shell
[host] $ fx sniff wlan
```

By default, this command captures packets for 30 seconds. To configure the duration, add the `--time {sec}` or `-t {sec}` option.

If you don't know the network interface name, run `fx sniff` without options. The error message shows you what interfaces are available. Alternatively, run:

```shell
[host] $ fx shell net if list       # Take note of `filepath` in output
```


### Show the hexdump of packets over the ethernet interface

```shell
[host] $ fx sniff --view hex eth
```

### Capture WLAN packets and store them in a file

```shell
[host] $ fx sniff --file my_packets wlan
```

The captured packets are first stored in the target's `/tmp/` directory. After the capture is complete, the files are moved to `//out/my_packets.pcapng` automatically.

### Stream out to Wireshark in realtime

**_NOTE:_** Linux only.

```shell
[host] $ fx sniff --view wireshark wlan
```

### Force stop
Packet capture runs for the specified duration (`--time` or `-t` option). If a user desires to stop early, presse one of the following keys:

```
c, q, C, Q
```
This will stop both a target side process and a host side process.

## How-to (on target device)

### Use netdump for debugging

`fx sniff` requires working `ssh` connectivity from the host to the target, which means that networking must be working to some degree. In some cases, networking might not be working at all. If you have access to the serial console while networking, including `ssh`, is not working, you must run `netdump` directly on the target. `netdump` provides a richer set of features than `fx sniff`.

#### Prerequisites

Before you use `netdump`, you must get the file path for the network interface. This is an example for WLAN interface (assuming your target device has one and only one WLAN interface - which is a typical case).

```shell
[target] $ iface_filepath=$(net if list wlan | grep filepath | while read c1 c2; do echo $c2; done)
```

#### Capture packets over the WLAN interface

```shell
[target] $ netdump -t 30 "$iface_filepath"
```

#### Show the hexdump of packets over the ethernet interface

```shell
[target] $ netdump --raw "$iface_filepath"
```

#### Stream out the binary dump in PCAPNG format

```shell
[target] $ netdump --pcapdump ${iface_filepath}
```

#### Capture packets and store them in a file

```shell
[target] $ netdump -t 30 -w /tmp/my_packets.pcapng "$iface_filepath"
```

#### Copy the dump file to the host

```shell
[host] $ cd ${FUCHSIA_OUT_DIR} && fx scp "[$(fx get-device-addr)]:/tmp/my_precious_packets.pcapng" .
```

#### `netdump` help

```shell
[target] $ netdump --help
```

#### Only Watch ARP, DHCP, and DNS packets

```shell
[target] $ netdump -t 10 -f "arp or port dns,dhcp" "$iface_filepath"
```

## Full syntax for filters
The packet filter language syntax is as follows. Keywords are in **bold**. Optional terms are in `[square brackets]`. Placeholders for literals are in `<angle brackets>`. Binary logical operators associate to the left. All keywords and port aliases should be in lower case.

<pre><code>
       expr ::= <b>(</b> expr <b>)</b>
              | <b>not</b> expr  | expr <b>and</b> expr | expr <b>or</b> expr
              | eth_expr  | host_expr     | trans_expr
length_expr ::= <b>greater</b> &lt;len&gt; | <b>less</b> &lt;len&gt;
       type ::= <b>src</b> | <b>dst</b>
   eth_expr ::= length_expr
              | <b>ether</b> [type] <b>host</b> &lt;mac_addr&gt;
              | [<b>ether</b> <b>proto</b>] net_expr
   net_expr ::= <b>arp</b>
              | <b>vlan</b>
              | <b>ip</b>  [length_expr | host_expr | trans_expr]
              | <b>ip6</b> [length_expr | host_expr | trans_expr]
  host_expr ::= [type] <b>host</b> &lt;ip_addr&gt;
 trans_expr ::= [<b>proto</b>] <b>icmp</b>
              | [<b>proto</b>] <b>tcp</b> [port_expr]
              | [<b>proto</b>] <b>udp</b> [port_expr]
              | port_expr
  port_expr ::= [type] <b>port</b> &lt;port_lst&gt;
</code></pre>

*   `<len>`: Packet length in bytes. Greater or less comparison is inclusive of `len`.
*   `<mac_addr>`: MAC address, e.g. `DE:AD:BE:EF:D0:0D`. Hex digits are case-insensitive.
*   `<ip_addr>`: IP address consistent with the IP version specified previously.
    E.g. `192.168.1.10`, `2001:4860:4860::8888`.
*   `<port_lst>`: List of ports or port ranges separated by commas, e.g. `13,ssh,6000-7000,20`.
    The following aliases for defined ports and port ranges can be used as items in the list, but
    not as part of a range (`3,dhcp,12` is allowed, `http-100` is not):

  Alias    | Port(s)
  :--------| :-------------------------
  `dhcp`   | `67-68`
  `dns`    | `53`
  `echo`   | `7`
  `ftpxfer`| `20`
  `ftpctl` | `21`
  `http`   | `80`
  `https`  | `443`
  `irc`    | `194`
  `ntp`    | `123`
  `sftp`   | `115`
  `ssh`    | `22`
  `telnet` | `23`
  `tftp`   | `69`
  `dbglog` | Netboot debug log port
  `dbgack` | Netboot debug log ack port

### Synonyms
The following aliases may be used instead of the keywords listed in the syntax:

Keyword | Alias
:-------| :----------
`ip`    | `ip4`
`port`  | `portrange`


## Reference: `fx` workflow packet signatures
There are many different kinds of services running between the Fuchsia
development host and the target. Those are usually invoked by `fx` commands.
Most of times, you are not interested in those packets generated by the `fx`
workflows. The following table lists noteworthy signatures.

| Use                  | Signature                    | Reference                                  |
|----------------------|------------------------------|--------------------------------------------|
| Logger               | port 33337                   | DEBUGLOG_PORT                              |
| Logger               | port 33338                   | DEBUGLOG_ACK_PORT                          |
| Bootserver           | port 33330                   | NB_SERVER_PORT                             |
| Bootserver           | port 33331                   | NB_ADVERT_PORT                             |
| Bootserver           | port 33332                   | NB_CMD_PORT_START                          |
| Bootserver           | port 33339                   | NB_CMD_PORT_END                            |
| Bootserver           | port 33340                   | NB_TFTP_OUTGOING_PORT                      |
| Bootserver           | port 33341                   | NB_TFTP_INCOMING_PORT                      |
| Package Server       | port 8083                    | docs/packages.md                           |
| fx shell             | port 22                      | devshell/shell                             |
| target netsvc addr   | fe80::xxxx:xxff:fexx:xxxx%XX | fx device-finder list --netboot            |
| host link-local addr | fe80::xxxx:xxxx:xxxx:xxxx%XX | fx device-finder list --ipv4=false --local |
| target netstack addr | fe80::xxxx:xxxx:xxxx:xxxx%XX | fx get-device-addr                         |
| zxdb                 | port 2345                    | devshell/contrib/debug                     |
| -                    | port 65026                   |                                            |
| -                    | port 65268                   |                                            |
| -                    | 1900                         |                                            |


## How do I test if `netdump` is broken?
You can run some sanity checks locally.

```shell
[host] $ fx set core.x64 --with //src/connectivity:tests,//src/connectivity/network/netdump:netdump_unit_tests
# (After running your target)
[host] $ fx test netdump_unit_test          # unit test
[host] $ fx test netdump_integration_tests  # integration test
```


## Troubleshooting

**_Q_** `fx sniff` commands give me the error `env: python3: No such file or directory`

**A** Please install Python 3 in your environment. Fuchsia is in the middle of migrating from Python 2.7 to Python 3.

**_Q_** I get the error `/boot/bin/sh: netdump not found`

**A** The `netdump` package is not prepared. Make sure to bundle `netdump` in the image. See [prepare the image](#prepare-image).
