/**
 *  https://en.wikipedia.org/wiki/IPv4
 */

#[derive(Debug)]
struct ipv4_header {
    // 4b: Version field. For IPv4, this is always equal to 4.
    // 4b: The Internet Header Length (IHL) field has 4 bits, which is the
    //  number of 32-bit words. Since an IPv4 header may contain a variable
    //  number of options, this field specifies the size of the header (this
    //  also coincides with the offset to the data).
    ver_ihl: u8,
    // 4b: Differentiated Services Code Point (formerly Type of Service)
    // 4b: Explicit Congestion Notification
    dscp_ecn: u8,
    // 16b: This 16-bit field defines the entire packet size in bytes,
    //  including header and data.
    length: u16,
    // 16b: This field is an identification field and is primarily used for
    //  uniquely identifying the group of fragments of a single IP datagram.
    identification: u16,
    // 3b: A three-bit field follows and is used to control or identify fragments.
    // 13b: The fragment offset field is measured in units of eight-byte blocks.
    //  It specifies the offset of a particular fragment relative to the beginning
    //  of the original unfragmented IP datagram.
    flags_fragof: u16,
    // 8b: field helps prevent datagrams from persisting (e.g. going in circles) on an internet.
    ttl: u8,
    // 8b: This field defines the protocol used in the data portion of the IP datagram.
    protocol: u8,
    // 16b: The 16-bit checksum field is used for error-checking of the header.
    //  Errors in the data field must be handled by the encapsulated protocol.
    checksum: u16,
    // 32b: This field is the IPv4 address of the sender of the packet.
    src: u32,
    // 32b: This field is the IPv4 address of the receiver of the packet.
    dest: u32,
    options: [u32; 4]
}