# send-arp
Send arp packet using rust libpnet

```
Send an arp packet.

By default an arp request packet will be sent.


USAGE:
    send-arp [FLAGS] -i <interface> --source-ip <source_ip> --source-mac <source_mac> --target-ip <target_ip> --target-mac <target_mac>

FLAGS:
        --reply      Send an arp reply packet
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <interface>                   Provide the interface to be used to send packets
        --source-ip <source_ip>      Set the source ip
        --source-mac <source_mac>    Set the source mac address
        --target-ip <target_ip>      Set the target ip
        --target-mac <target_mac>    Set the target mac address
```

**Example:**

`send-arp --source-ip 192.168.0.2 --source-mac ac:2e:4f:cd:cd:ae --target-ip 192.168.0.1 --target-mac ef:34:23:ff:ad:dd`
