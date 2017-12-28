use std::net::IpAddr;
use ipnet::{IpNet, Ipv4Net, Ipv6Net, Ipv4Subnets, Ipv6Subnets};
use iprange::IpRange;
use std::iter::FromIterator;

pub enum IpRangeUniversal{
    V4(IpRange<Ipv4Net>),
    V6(IpRange<Ipv6Net>)
}

fn parse_single_ip(txt: &str) -> Result<IpAddr, String>{
    return match txt.parse::<IpAddr>() {
        Ok(val) => Ok(val),
        Err(err) =>  Err(format!("Could not parse IP from {} because of {}", txt, err))
    }
}

pub fn parse_ip(txt: &str) -> Result<IpRangeUniversal, String> {
    //TODO: enable
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
                    IpAddr::V4(end) => Ok({
                        let subnets = Ipv4Subnets::new(begin, end, 0);
                        IpRangeUniversal::V4(IpRange::from_iter(subnets))
                    }),
                    IpAddr::V6(end) => Err(format!("Mixed IP versions in range {}-{}", begin, end))
                }
            },
            IpAddr::V6(begin) => {
                match end {
                    IpAddr::V4(end) => Err(format!("Mixed IP versions in range {}-{}", begin, end)),
                    IpAddr::V6(end) => Ok({
                        let subnets = Ipv6Subnets::new(begin, end, 0);
                        IpRangeUniversal::V6(IpRange::from_iter(subnets))
                    })
                }
            }
        }

    } else  if let Some(_) = txt.find("/") {
    //network
        let net:IpNet = match txt.parse() {
            Ok(val) => val,
            Err(err) => return Err(format!("Could not parse IP network from {} because of {}", txt, err))
        };
        return match net{
            IpNet::V4(val) => Ok(IpRangeUniversal::V4({
                let mut range = IpRange::new();
                range.add(val);
                range
            })),
            IpNet::V6(val) => Ok(IpRangeUniversal::V6({
                let mut range = IpRange::new();
                range.add(val);
                range
            }))
        }

    } else {
    //single IP
        let ip = parse_single_ip(txt)?;
        return match ip {
            IpAddr::V4(addr) => Ok(IpRangeUniversal::V4({
                let net = Ipv4Net::new(addr, 32).unwrap();
                let mut range = IpRange::new();
                range.add(net);
                range
            })),
            IpAddr::V6(addr) => Ok(IpRangeUniversal::V6({
                let net = Ipv6Net::new(addr, 128).unwrap();
                let mut range = IpRange::new();
                range.add(net);
                range
            }))
        }
    }
}

pub fn validate_ip(txt: String) -> Result<(), String> {
    match parse_ip(&txt) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

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
}