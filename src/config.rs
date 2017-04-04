use std::net::Ipv4Addr;
use pnet::util::MacAddr;


/// Config for storing the cli options.
#[derive(Debug)]
pub struct Config {
    pub source_ip: Ipv4Addr,
    pub source_mac: MacAddr,
    pub target_ip: Ipv4Addr,
    pub target_mac: MacAddr
}