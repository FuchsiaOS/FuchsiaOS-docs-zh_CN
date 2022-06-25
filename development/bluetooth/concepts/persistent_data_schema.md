# Bluetooth persistent data schema

The Fuchsia Bluetooth subsystem persists bonding information for any peer that has been successfully
associated with Fuchsia using the standard Bluetooth pairing procedures. Bonding information
includes pairing secrets (such as encryption keys) as well as metadata about the peer used to
establish a connection.

The persistent bonding storage is maintained by the
[`bt-gap`](/src/connectivity/bluetooth/core/bt-gap) component. The data is stored using the
[`stash`](/sdk/fidl/fuchsia.stash/) library. There are two kinds of persisted data: Host Data and
Bonding Data.

## JSON schema

The core Bluetooth system stores its persisted data in JSON format. This section describes
the format used to serialized the data.

### Basic types

This section describes the schema for types that are reused by other schemas. All schemas are
represented as JSON objects.

#### Address {#address}

Represents a Bluetooth Device Address

Key   | Value Type               | Description
------|--------------------------|-----------------------
type  | String                   | `public` or `random`
value | Array of Number (8-bits) | 6-octet device address in little-endian byte order

The following example represents a random address with value `FF:FF:00:00:00:01`:

```
{
   "type": "random"
   "value": [1, 0, 0, 0, 255, 255]
}
```

#### Security properties {#security-properties}

The security properties of a Bluetooth connection under which pairing secrets were exchanged. For
example, an Identity Resolving Key that was distributed by a peer over a link encrypted with an
unauthenticated Temporary Key is considered to be "unauthenticated".

Key               | Value Type     | Description
------------------|----------------|--------------------------------------------------------------------
authenticated     | Boolean        | `true` if MITM protection was enabled
secureConnections | Boolean        | `true` if the secret was exchanged using Secure Connections pairing
encryptionKeySize | Number (8-bit) | Size of the encryption key used for the exchange

Example:

```
{
    "authenticated": false,
    "secureConnections": true,
    "encryptionKeySize": 16
}
```

#### Key {#key}

128-bit number that represents a secret associated with pairing. A common key that uses this schema
is the local device's Identity Resolving Key (IRK).

When used by itself, a Key represents a locally generated key stored for future distribution.

Key   | Value Type                                  | Description
------|---------------------------------------------|-----------------------------------------
value | Array of Number (8-bits)                    | 16-octet key in little-endian byte order

#### Peer key {#key}

128-bit key that represents a pairing-related secret from the peer. A common key that uses this
schema is the peer's Identity Resolving Key (IRK).

Each key has a 128-bit value (like [Key](#key)) and [security properties](#security-properties),
which represent the security of the connection when the key was distributed.

Key      | Value Type                                  | Description
---------|---------------------------------------------|-----------------------------------------
security | [Security properties](#security-properties) | Link security during distribution
value    | Array of Number (8-bits)                    | Underlying 16-octet key in little-endian byte order

The following example represents a key with MITM protection of value `0x100f0e0d0c0b0a090807060504030201`:

```
{
    "security": {
        "authenticated": true,
        "secureConnections": false,
        "encryptionKeySize": 16
    },
    "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
}
```

#### Long term key {#ltk}

128-bit key used to establish an encrypted connection. This schema is used to represent the Low
Energy Long Term Key (LTK).

The security properties of a Long Term Key's peer key represent not only the security of the
connection used to distribute and generate the key, but also the security of the key itself.

Key  | Value Type                | Description
-----|---------------------------|---------------------
key  | [Peer key](#peer-key)     | The encryption key.
ediv | Number (16-bit)           | Encrypted Diversifier
rand | Number (64-bit)           | Random value

Example:

```
{
    "key": {
        "security": {
            "authenticated": true,
            "secureConnections": false,
            "encryptionKeySize": 16
        },
        "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    },
    "ediv": 48879,
    "rand": 9223372036854775807
}
```

### Host data

The Fuchsia Bluetooth system is compatible with one or more Bluetooth controllers. The system
instantiates a Bluetooth Host Subsystem ([bt-host](/src/connectivity/bluetooth/core/bt-host))
for every available controller. Fuchsia stores some metadata for a bt-host that is in use.

Key             | Value Type          | Description
----------------|---------------------|-----------------------
irk (optional)  | [Key](#key)         | Identity Resolving Key that is distributed to all peers of the bt-host

Example:

```
{
    "irk": {
        "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }
}
```

### Bonding data

Every peer that has been associated with the Fuchsia system using a standard Bluetooth pairing
procedure is considered to have a bond with the Fuchsia system. Fuchsia persists metadata for each
bonded peer to re-establish an encrypted link on future connections without needing to repeat
security and pairing procedures. Fuchsia also persists additional metadata to speed up the
connection establishment process.

Separate JSON object schemas are defined for LE and BR/EDR transports. The Fuchsia system may
persist LE data, BR/EDR data, or both based on peer support for the particular transport (e.g. a
dual-mode peer may result in bonding data for both transports).

#### LE connection parameters {#le-connection-params}

Key                | Value Type      | Description
-------------------|-----------------|------------------
connectionInterval | Number (16-bit) | Connection interval in controller timeslices
connectionLatency  | Number (16-bit) | Connection latency in controller timeslices
supervisionTimeout | Number (16-bit) | Supervision timeout in controller timeslices

#### LE bond data {#le-data}

Key                             | Value Type                                         | Description
--------------------------------|----------------------------------------------------|----------------------------------------------
connectionParameters (optional) | [LE Connection Parameters](#le-connection-params)  | The peer's preferred connection parameters
peerLtk (optional)              | [Long Term Key](#ltk)                              | Long Term Key distributed by the peer
localLtk (optional)             | [Long Term Key](#ltk)                              | Long Term Key generated locally and distributed to the peer
irk (optional)                  | [Peer key](#peer-key)                              | The peer's Identity Resolving Key

#### BR/EDR bond data {#bredr-data}

Key                       | Value Type            | Description
--------------------------|-----------------------|-----------------------------------------------------------
rolePreference (optional) | String                | `"leader"` or `"follower"`
linkKey (optional)        | [Peer key](#peer-key) | The link encryption key
services                  | Array of String       | Cached discovered service UUIDs

#### Bonding data schema

Key              | Value Type                      | Description
-----------------|---------------------------------|------------------------
identifier       | Number                          | 64-bit opaque peer ID
address          | [Address](#address)             | The identity address of the peer
hostAddress      | [Address](#address)             | The identity address of the local bt-host the peer is bonded to
name (optional)  | String                          | The complete or short "local name" of the peer.
le (optional)    | [LE bond data](#le-data)        | Bonding data for the Low Energy transport
bredr (optional) | [BR/EDR bond data](#bredr-data) | Bonding data for the BR/EDR transport

Example:

```
{
    "identifier": 1,
    "address: {
        "type": "random",
        "value": [202, 202, 254, 202, 239, 190]
    },
    "hostAddress": {
        "type": "public",
        "value": [1, 2, 3, 4, 5, 6]
    },
    "name": "My Device",
    "le": {
        "peerLtk": {
            "key": {
                "security": {
                    "authenticated": true,
                    "secureConnections": false,
                    "encryptionKeySize": 16
                },
                "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
            },
            "ediv": 0,
            "rand": 0,
        },
    },
    "bredr": {
        "rolePreference": "follower",
        "linkKey": {
            "security": {
                "authenticated": true,
                "secureConnections": true,
                "encryptionKeySize": 16
            },
            "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        },
        "services": [
           "0000110a-0000-1000-8000-00805f9b34fb",
           "0000110b-0000-1000-8000-00805f9b34fb"
        ]
    }
}
```
