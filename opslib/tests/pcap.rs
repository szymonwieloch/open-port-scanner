extern crate opslib;
use opslib::rawsock::{PCap, RawSock};


#[test]
fn load_pcap () {
    let pcap = PCap::open_default_locations().expect("Could not open pcap library");
    unsafe {pcap.open_interface("wlp2s0")}.expect("Could not open interface");
}