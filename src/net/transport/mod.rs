pub mod icmp;

#[derive(Debug)]
enum TransportPacket {
    ICMP(icmp::ICMPPacket)
}