
# Open Port Scanner
Open Port Scanner - effective replacement for nmap and masscan.

## Why another port scanner?
There are several applications that you can use for port scanning, most notably **nmap** and **masscan**.
There are however several problems with both:
* nmap work very well, but is very, very slow. Totally unpractical for big networks or huge sets of ports.
* masscan is super fast but has multiple bugs. The main refere to retransmissions and scanning UDP ports. Masscan unfortunately does not support IPv6.
* None of mentioned applications supports asynchronous adding of new networks and per-IP limits (you can accidentally DOS a device).
* Both nmap and masscan have GPL licenses that limit what you can do with them.

The goal of this project is to create a new library/binary that will allow users to scan ports of known IPs and fix the mentioned issues.
