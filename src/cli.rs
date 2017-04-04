use std::net::{Ipv4Addr, AddrParseError};
use pnet::util::{MacAddr, ParseMacAddrErr};
use clap::{Arg, App};

use config::Config;

pub fn cli_main() -> Config {
    let matches = App::new("send-arp")
                          .version("0.1.0")
                          .author("Dineshs91 <dineshpy07@gmail.com>")
                          .about("Send an arp reply packet")
                          .arg(Arg::with_name("source_ip")
                              .long("source-ip")
                              .required(true)
                              .takes_value(true)
                              .help("Set the source ip"))
                          .arg(Arg::with_name("source_mac")
                              .long("source-mac")
                              .required(true)
                              .takes_value(true)
                              .help("Set the source mac address"))
                          .arg(Arg::with_name("target_ip")
                              .long("target-ip")
                              .required(true)
                              .takes_value(true)
                              .help("Set the target ip"))
                          .arg(Arg::with_name("target_mac")
                              .long("target-mac")
                              .required(true)
                              .takes_value(true)
                              .help("Set the target mac address"))
                          .get_matches();

    let source_ip: Result<Ipv4Addr, AddrParseError> = matches.value_of("source_ip").unwrap()
                                                      .trim().parse();
    let source_mac: Result<MacAddr, ParseMacAddrErr> = matches.value_of("source_mac").unwrap()
                                                       .trim().parse();
    let target_ip: Result<Ipv4Addr, AddrParseError> = matches.value_of("target_ip").unwrap()
                                                      .trim().parse();
    let target_mac: Result<MacAddr, ParseMacAddrErr> = matches.value_of("target_mac").unwrap()
                                                       .trim().parse();

    let config: Config = Config {
        source_ip: source_ip.unwrap(),
        source_mac: source_mac.unwrap(),
        target_ip: target_ip.unwrap(),
        target_mac: target_mac.unwrap()
    };

    // return cli config.
    config
}