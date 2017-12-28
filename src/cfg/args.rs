use clap::{Arg, App};
use super::ports::{parse_ports, validate_ports};
use super::ips::{parse_ip, validate_ip, IpRangeUniversal};
use iprange::IpRange;
use ipnet::{Ipv4Net, Ipv6Net};
use range_map::RangeSet;

#[derive(Debug)]
pub struct OpsConfig {
    tcp_ports: RangeSet<u16>,
    udp_ports: RangeSet<u16>,
    ipv4: IpRange<Ipv4Net>,
    ipv6: IpRange<Ipv6Net>,
    ping: bool,
    max_rate: Option<u32>,
    max_dev_rate: Option<u32>
}

const TCP_PORTS: &str = "tcp-ports";
const UDP_PORTS: &str = "udp-ports";
const PING_ICMP: &str = "ping-icmp";
const MAX_RATE: &str = "max-rate";
const MAX_DEV_RATE: &str = "max-dev-rate";
//const IPS: & str = "ips";
const NONE: &str = "none";


pub fn parse_args() -> OpsConfig{
    let matches = App::new("Open Port Scanner")
        .version("0.1.0") //TODO: get it from metadata
        .author("Szymon Wieloch <szymon.wieloch@gmail.com>")
        .about("Scans devices and networks, finds open ports")
        .arg(Arg::with_name(TCP_PORTS)
            .short("t")
            .long(TCP_PORTS)
            .value_name("PORTS")
            .default_value("0-1023")
            .help("List of ports in nmap-compatible format")
            .validator(validate_ports)
            .takes_value(true))
        .arg(Arg::with_name(UDP_PORTS)
            .short("u")
            .long(UDP_PORTS)
            .value_name("PORTS")
            .default_value("")
            .validator(validate_ports)
            .help("List of ports in nmap-compatible format")
            .takes_value(true))
        .arg(Arg::with_name(PING_ICMP)
            .short("p")
            .long(PING_ICMP)
            .help("Enables ICMP pings"))
        .arg(Arg::with_name(MAX_RATE)
            .short("r")
            .long(MAX_RATE)
            .takes_value(true)
            .default_value("100")
            .help("Sets the maximum packet rate"))
        .arg(Arg::with_name(MAX_DEV_RATE)
            .short("d")
            .long(MAX_DEV_RATE)
            .takes_value(true)
            .help("Sets the maximum packet rate per device"))
        .arg(Arg::with_name("ips")
            .index(1)
            .value_name("IPS")
            .multiple(true)
            .required(true)
            .validator(validate_ip)
            .help("List of IPs, networks or IP ranges that should be scanned"))
        .get_matches();

    //unwrap is now safe because of previous validation
    let ip_iter = matches.values_of("ips").unwrap().map(|t| parse_ip(t).unwrap());
    let mut ipv4: IpRange<Ipv4Net> = IpRange::new();
    let mut ipv6: IpRange<Ipv6Net> = IpRange::new();
    for range in ip_iter {
        match range {
            IpRangeUniversal::V4(r) => ipv4 = ipv4.merge(&r),
            IpRangeUniversal::V6(r) => ipv6 = ipv6.merge(&r),
        }
    }
    OpsConfig{
        tcp_ports: parse_ports(matches.value_of(TCP_PORTS).unwrap()).unwrap(), //safe because of validation
        udp_ports: parse_ports(matches.value_of(UDP_PORTS).unwrap()).unwrap(), //safe because of validation
        ipv4: ipv4,
        ipv6: ipv6,
        ping: matches.is_present(PING_ICMP),
        max_rate: if matches.value_of(MAX_RATE).unwrap() == NONE {
            None
        } else {
            Some(value_t!(matches, MAX_RATE, u32).unwrap_or_else(|e|e.exit()))
        },
        max_dev_rate: if matches.is_present(MAX_DEV_RATE) {
            Some(value_t!(matches, MAX_DEV_RATE, u32).unwrap_or_else(|e|e.exit()))
        }
        else {
            None
        }
    }
}