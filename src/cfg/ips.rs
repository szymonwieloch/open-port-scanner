use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use ipnetwork::{Ipv4Network, Ipv6Network, IpNetwork};
use std::str::FromStr;
use opslib::utils::{Range};

#[derive(Debug)]
pub enum IpRange{
    V4(Range<Ipv4Addr>),
    V6(Range<Ipv6Addr>)
}

fn parse_single_ip(txt: &str) -> Result<IpAddr, String>{
    return match txt.parse::<IpAddr>() {
        Ok(val) => Ok(val),
        Err(err) =>  Err(format!("Could not parse IP from {} because of {}", txt, err))
    }
}

pub fn parse_ip(txt: &str) -> Result<IpRange, String> {
    if let Some(idx) = txt.find("-") {
    //range
        let begin = &txt[0..idx];
        let end = &txt[idx+1..];
        let begin = parse_single_ip(begin)?;
        let end = parse_single_ip(end)?;
        if begin>end{
            return Err(format!("{} is greater than {}", begin, end))
        }
        return match begin {
            IpAddr::V4(begin) =>{
                match end {
                    IpAddr::V4(end) => Ok(IpRange::V4(Range::new(begin, end))),
                    IpAddr::V6(end) => Err(format!("Mixed IP versions in range {}-{}", begin, end))
                }
            },
            IpAddr::V6(begin) => {
                match end {
                    IpAddr::V4(end) => Err(format!("Mixed IP versions in range {}-{}", begin, end)),
                    IpAddr::V6(end) => Ok(IpRange::V6(Range::new(begin, end)))
                }
            }
        }

    } else if let Some(_) = txt.find("/") {
    //network
        let net:IpNetwork = match txt.parse() {
            Ok(val) => val,
            Err(err) => return Err(format!("Could not parse IP network from {} because of {}", txt, err))
        };
        return match net{
            IpNetwork::V4(val) => Ok(IpRange::V4(Range::new(val.ip(), val.broadcast()))),
            IpNetwork::V6(val) => Ok(IpRange::V6(Range::new(val.ip(), ipv6_broadcast(&val))))
        }

    } else {
    //single IP
        let ip = parse_single_ip(txt)?;
        return match ip{
            IpAddr::V4(val) => Ok(IpRange::V4(Range::new(val, val))),
            IpAddr::V6(val) => Ok(IpRange::V6(Range::new(val, val)))
        }
    }
}

pub fn validate_ip(txt: String) -> Result<(), String> {
    match parse_ip(&txt) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

//This functionality is not correctly implemented in ipnetwork
//This could be done by a trait
//TODO remove this once ipnetwork gets a better implementation
fn ipv6_broadcast(net: &Ipv6Network) -> Ipv6Addr {
    let mut segments = net.ip().segments();
    for (i, segment) in net.mask().segments().iter().enumerate(){
        segments[i] |= !*segment;
    }
    Ipv6Addr::from(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_ip_ok() {
        let result = parse_single_ip("172.56.33.7");
        let expected = Ok(IpAddr::V4(Ipv4Addr::new(172, 56, 33, 7)));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_single_ip_err() {
        let result = parse_single_ip("172.56.3c.7");
        let expected = Err(String::from("Could not parse IP from 172.56.3c.7 because of invalid IP address syntax"));
        assert_eq!(result, expected);
    }

    #[test]
    fn ipv6_broadcast_small() {
        let net = "fe::/123".parse::<Ipv6Network>().unwrap();
        let result = ipv6_broadcast(&net);
        let expected = "fe::1f".parse::<Ipv6Addr>().unwrap();
        assert_eq!(result, expected);
    }

    fn ipv6_broadcast_large() {
        let net = "ffa::/19".parse::<Ipv6Network>().unwrap();
        let result = ipv6_broadcast(&net);
        let expected = "ff:bf:ff:ff:ff:ff:ff:ff".parse::<Ipv6Addr>().unwrap();
        assert_eq!(result, expected);
    }
}