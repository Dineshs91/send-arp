extern crate pnet;
extern crate clap;

pub mod cli;
pub mod config;

use std::thread;
use std::time::Duration;
use std::net::Ipv4Addr;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::arp::MutableArpPacket;
use pnet::util::MacAddr;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::MutablePacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpOperation};

use config::Config;
use cli::cli_main;


fn send_arp_packet(interface: NetworkInterface, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr, arp_operation: ArpOperation) {
    let(mut tx, _) = match datalink::channel(&interface, Default::default()) {
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
    ///     target_proto_addr: [0xc0, 0xa8, 0x00, 0x65], // Ipv4(192.168.0.101)
    ///     payload: [],
    /// }

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(arp_operation);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    tx.send_to(&ethernet_packet.to_immutable(), Some(interface));
}

fn main() {
    let config: Config = cli_main();
    let mut packet_count: i32 = 0;

    loop {
        let interfaces = datalink::interfaces();

        let interfaces_name_match = |iface: &NetworkInterface| iface.name == config.interface;
        let interface = interfaces.into_iter().filter(interfaces_name_match).next().unwrap();
       
        send_arp_packet(interface, config.source_ip, config.source_mac, config.target_ip, config.target_mac, config.arp_operation);

        packet_count += 1;
        let arp_operation = match config.arp_operation {
            ArpOperations::Request => "Request",
            ArpOperations::Reply => "Reply",
            ArpOperation(_) => panic!("Unknown operation")
        };
        println!("Sent {} ARP {} packets.", packet_count, arp_operation);
        thread::sleep(Duration::new(5, 0));
    }
}
