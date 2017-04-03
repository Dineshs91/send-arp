extern crate pnet;

use std::io;
use std::thread;
use std::time::Duration;
use std::net::Ipv4Addr;
use std::net::AddrParseError;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::arp::MutableArpPacket;
use pnet::util::{MacAddr, ParseMacAddrErr};
use pnet::packet::ethernet::{EtherTypes, EtherType};
use pnet::packet::MutablePacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpOperation};


fn send_arp_request_packet() {
    let interfaces = datalink::interfaces();

    // Get en1 interface in osx.
    let interfaces_name_match = |iface: &NetworkInterface| iface.name == "en1";
    let interface = interfaces.into_iter().filter(interfaces_name_match).next().unwrap();

    let(mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    /// ethernet_packet = Ethernet {
    ///     destination: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    ///     source: [0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b],
    ///     ethertype: [0x08, 0x06], // Arp(0x0806)
    ///     payload: arp_packet
    /// }
    ///
    /// arp_packet = Arp {
    ///     hardware_type: [0x00, 0x01],
    ///     protocol_type: [0x08, 0x00], // Ipv4(0x0800)
    ///     hw_addr_len: [0x06],
    ///     proto_addr_len: [0x04],
    ///     operation: [0x00, 0x01], // Request(0x0001)
    ///     sender_hw_addr: [0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b],
    ///     sender_proto_addr: [0xc0, 0xa8, 0x00, 0x66], // Ipv4(192.168.0.102)
    ///     target_hw_addr: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff], // Broadcast
    ///     target_proto_addr: [0xc0, 0xa8, 0x00, 0x65], // // Ipv4(192.168.0.101)
    ///     payload: [],
    /// }

    let buffer: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b, 0x08, 0x06,
                         0x00, 0x01, 0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b, 0xc0, 0xa8, 0x00, 0xbf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0xa8, 0x00, 0x01];
    let ethernet_packet = EthernetPacket::new(&buffer);
    tx.send_to(&ethernet_packet.unwrap(), Some(interface));
}

fn send_arp_reply_packet(gateway: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr) {
    let interfaces = datalink::interfaces();

    // Get en1 interface in osx.
    let interfaces_name_match = |iface: &NetworkInterface| iface.name == "en1";
    let interface = interfaces.into_iter().filter(interfaces_name_match).next().unwrap();

    let(mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    /// ethernet_packet = Ethernet {
    ///     destination: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    ///     source: [0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b],
    ///     ethertype: [0x08, 0x06], // Arp(0x0806)
    ///     payload: arp_packet
    /// }
    ///
    /// arp_packet = Arp {
    ///     hardware_type: [0x00, 0x01],
    ///     protocol_type: [0x08, 0x00], // Ipv4(0x0800)
    ///     hw_addr_len: [0x06],
    ///     proto_addr_len: [0x04],
    ///     operation: [0x00, 0x02], // Reply(0x0002)
    ///     sender_hw_addr: [0x28, 0xef, 0xf9, 0x5f, 0x8e, 0x2b],
    ///     sender_proto_addr: [0xc0, 0xa8, 0x00, 0x66], // Ipv4(192.168.0.102)
    ///     target_hw_addr: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff], // Broadcast
    ///     target_proto_addr: [0xc0, 0xa8, 0x00, 0x65], // // Ipv4(192.168.0.101)
    ///     payload: [],
    /// }

    let buffer: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();
    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_packet = MutableArpPacket::new(ethernet_packet.payload_mut()).unwrap();
    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Arp);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Reply);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(gateway);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    tx.send_to(&arp_packet, Some(interface));
}

fn send_gratitious_arp_request() {
    // TODO:
}


fn main() {
    // let mut mac_addr = String::new();
    // io::stdin().read_line(&mut mac_addr).expect("Failed to read input");

    // let mac_addr: Result<MacAddr, ParseMacAddrErr> = mac_addr.trim().parse();
    // println!("Mac address is {:?}", mac_addr);

    // let mut ip_addr = String::new();
    // io::stdin().read_line(&mut ip_addr).expect("Failed to read input");

    // let ip_addr: Result<Ipv4Addr, AddrParseError> = ip_addr.trim().parse();
    // println!("Ip address is {:?}", ip_addr);

    // Get the following
    //   1. src - gateway_ip (gateway)
    //   2. hwsrc - my_mac  (source-mac)
    //   3. dst - target_ip  (target-ip)
    //   4. hwdst - target_mac  (target-mac)
    //
    // Ethernet
    //   1. src - my_mac
    //   2. dst - target_mac

    // // loop {
    // //     send_arp_reply_packet();
    // //     thread::sleep(Duration::new(2, 0));
    // // }
    let gateway: Result<Ipv4Addr, AddrParseError> = "127.0.0.1".parse();
    let target_ip: Result<Ipv4Addr, AddrParseError> = "127.0.0.1".parse();
    let source_mac: Result<MacAddr, ParseMacAddrErr> = "ef::ef::ef::ef::ef::ef".parse();
    let target_mac: Result<MacAddr, ParseMacAddrErr> = "ef::ef::ef::ef::ef::ef".parse();
    send_arp_reply_packet(gateway.unwrap(), source_mac.unwrap(), target_ip.unwrap(), target_mac.unwrap());
}
