use range_map::{Range, RangeSet};

///Parses only single port value.
fn parse_port(port: &str) -> Result<u16, String> {
    match port.parse::<u16>() {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Could not parse port value: \"{}\" because {}", port, err))
    }
}

///Parses set of ports in nmap-compabile format.
///Returns human-friendly message on error for use in parsing command line arguments.
pub fn parse_ports(txt: &str) -> Result<RangeSet<u16>, String> {
    if txt.is_empty() {
        return Ok(RangeSet::new())
    }
    txt.split(",").map(parse_range).collect()
}

fn parse_range(txt: &str) -> Result<Range<u16>, String> {
    match txt.find("-"){
        Some(idx) => {
            let start = &txt[0..idx];
            let end = &txt[idx+1..];
            let start = parse_port(start)?;
            let end = parse_port(end)?;
            if start > end {
                return Err(format!("{} is greater than {}", start, end));
            }
            Ok(Range::new(start, end))
        },
        None => {
            let val = parse_port(txt)?;
            Ok(Range::single(val))
        }
    }
}

pub fn validate_ports(port: String) -> Result<(), String> {
    match parse_ports(&port) {
        Ok(_) => Ok(()),
        Err(descr) => Err(descr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn parse_empty(){
        let result = parse_ports("");
        let expected = Ok(RangeSet::new());
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_single_port(){
        let result = parse_ports("5");
        let expected = Ok(RangeSet::single(5));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_single_range(){
        let result = parse_ports("5-8");
        let expected = Ok(RangeSet::from_iter(vec![Range::new(5, 8)]));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_multiple(){
        let result = parse_ports("5-8,2,45-80");
        let expected = Ok(RangeSet::from_iter(vec![Range::new(2, 2), Range::new(5, 8), Range::new(45, 80)]));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_multiple_and_merge(){
        let result = parse_ports("5-10,9-18,7");
        let expected = Ok(RangeSet::from_iter(vec![Range::new(5, 18)]));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_error_not_num(){
        let result = parse_ports("blah");
        let expected = Err(String::from("Could not parse port value: \"blah\" because invalid digit found in string"));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_error_too_big(){
        let result = parse_ports("100000");
        let expected = Err(String::from("Could not parse port value: \"100000\" because number too large to fit in target type"));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_error_reverted_range(){
        let result = parse_ports("10-5");
        let expected = Err(String::from("10 is greater than 5"));
        assert_eq!(result, expected);
    }


}