PCAP
=====

PCAP is a rust based library that uses no dependancies or libraries to read and capture the packets from your network interface on your PC. This project also allows you to get the packets
in a decoded manor. For example instead of passing a u8 buf / byte array we pass Ethernet Frame > IPv4 Layer > UDP Header > Payload. All of these packets can be re-encoded back into bytes.

- No Libraries / Crates used

Supported Packets
-----
| Layer | Name | Status |
| --- | --- | --- |
| 2 | Ethernet Frames | Complete |
| 2.5 | ARP | Complete |
| 2.5 | Broadcast | Partial |
| 3 | IPv4 | Complete |
| 3 | IPv6 | Complete |
| 3 | GRE | - |
| 3.5 | ICMP | Complete |
| 3.5 | ICMPv6 | Complete |
| 4 | UDP | Complete |
| 4 | TCP | Complete |
| 5 | DHCP | Partial |
| 5 | M/DNS | Partial |
| 5 | Quic | - |
| 5 | TLSv1.2 | - |
| 5 | SSH | - |
| 5 | POP/IMAP | - |
| 5 | SMTP | - |
| 5 | uTP | - |
| 5 | BitTorrent DHT | - |
| 5 | BitTorrent Tracker | - |
| 5/6 | HTTP | - |


OS Support
-----
| OS | Supported |
| --- | --- |
| Linux | COMPLETE |
| MacOS | Partial |
| Windows | Partial |
