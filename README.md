# send-arp
Send arp packet using rust libpnet

```
Send an arp packet.

By default an arp request packet will be sent.


USAGE:
    send-arp [FLAGS] --interface <interface> --source-ip <source_ip> --source-mac <source_mac> --target-ip <target_ip> --target-mac <target_mac>

FLAGS:
        --reply      Send an arp reply packet
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <interface>      Provide the interface to be used to send packets
        --source-ip <source_ip>      Set the source ip
        --source-mac <source_mac>    Set the source mac address
        --target-ip <target_ip>      Set the target ip
        --target-mac <target_mac>    Set the target mac address
```

### Send arp request

This is the default behaviour.

`send-arp -i en1 --source-ip 192.168.0.2 --source-mac ac:2e:4f:cd:cd:ae --target-ip 192.168.0.1 --target-mac ef:34:23:ff:ad:dd`

### Send arp reply

`send-arp -i en1 --source-ip 192.168.0.2 --source-mac ac:2e:4f:cd:cd:ae --target-ip 192.168.0.1 --target-mac ef:34:23:ff:ad:dd --reply`

### Send Gratuitous arp (Request)

- Source and target ip will be the ip of the source (The host issuing the arp)
- Target mac is the broadcast mac address (`ff:ff:ff:ff:ff:ff`)

`send-arp -i en1 --source-ip 192.168.0.2 --source-mac ac:2e:4f:cd:cd:ae --target-ip 192.168.0.2 --target-mac ff:ff:ff:ff:ff:ff`

### Blog post

http://blog.dineshs91.com/post/send_arp/
