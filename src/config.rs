use std::net::Ipv4Addr;
use pnet::util::MacAddr;
use pnet::packet::arp::ArpOperation;


/// Config for storing the cli options.
#[derive(Debug)]
pub struct Config {
    pub interface: String,
    pub source_ip: Ipv4Addr,
    pub source_mac: MacAddr,
    pub target_ip: Ipv4Addr,
    pub target_mac: MacAddr,
    pub arp_operation: ArpOperation
}