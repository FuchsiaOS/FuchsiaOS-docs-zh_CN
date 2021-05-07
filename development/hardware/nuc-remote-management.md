## Remote management of NUC devices

To enable remote management, including KVM, you need to configure Intel AMT (Active Management Technology).

Note: This assumes you're using NUC connected to the EdgeRouter. If
your networking setup is different, you may need a different network
configuration.

1. Enter Intel ME settings by pressing Ctrl+P on the boot screen.
    + The first time you need to set a password, the default one is "admin". Password must be at
     least 8 characters long, contain both lowercase and uppercase characters, at least one
     digit and at least one non alpha-numeric character.

1. Configure network
    + Go to Network Setup > TCP/IP Settings > Wired LAN IPV4 Configuration.
    + Disable __DHCP Mode__ and set a static __IPV4 Address__.
    + Return to AMT Configuration and enable __Activate Network Access__.
    + Exit Intel ME settings and save your changes.

The Intel AMT serial-over-LAN and vPro KVM needs to be enabled before
use. These are enabled using the `wsman` command-line utility.

These instructions assume you have set the `AMT_HOST` variable, which
contains the IPv4 address you configured in the Intel ME settings,
In these instructions, `AMT_PASSWORD` is the Intel ME password and `VNC_PASSWORD`
is the VNC password.

Note: Password must be _exactly_ 8 characters long, contain both lowercase and
uppercase characters, at least one digit and at least one non alpha-numeric
character.

#### Intel AMT serial-over-LAN

1. Enable AMT redirection service:

    ```
    wsman put http://intel.com/wbem/wscim/1/amt-schema/1/AMT_RedirectionService -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k ListenerEnabled=true
    ```

Now, you can remotely access the NUC using [amtterm](https://git.kraxel.org/cgit/amtterm/): `amtterm -u admin -p ${AMT_PASWORD} ${AMT_HOST}`.

#### Intel vPro KVM

1. Set the VNC password:

   ```
   wsman put http://intel.com/wbem/wscim/1/ips-schema/1/IPS_KVMRedirectionSettingData -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k RFBPassword=${VNC_PASSWORD}
   ```

2. Enable KVM redirection to port 5900:

   ```
   wsman put http://intel.com/wbem/wscim/1/ips-schema/1/IPS_KVMRedirectionSettingData -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k Is5900PortEnabled=true
   ```

3. Disable opt-in policy (do not ask user for console access):

   ```
   wsman put http://intel.com/wbem/wscim/1/ips-schema/1/IPS_KVMRedirectionSettingData -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k OptInPolicy=false
   ```

4. Disable session timeout:

   ```
   wsman put http://intel.com/wbem/wscim/1/ips-schema/1/IPS_KVMRedirectionSettingData -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k SessionTimeout=0
   ```

5. Enable KVM:

   ```
   wsman invoke -a RequestStateChange http://schemas.dmtf.org/wbem/wscim/1/cim-schema/2/CIM_KVMRedirectionSAP -h ${AMT_HOST} -P 16992 -u admin -p ${AMT_PASSWORD} -k RequestedState=2
   ```

Now, you can remotely access the NUC using any VNC client, for example using VNC: `vncviewer ${AMT_HOST}`.
