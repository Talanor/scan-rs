/*
    Shamelessly overly-inspired from:
    https://github.com/MrKiven/rping/blob/master/src/main.rs

    Protocol RFC: https://tools.ietf.org/html/rfc792
*/

#[allow(dead_code)] // WTF rust??
struct ICMPPacket {
    typ: u8,
    code: u8,
    checksum: u16,
    roh: u32
}

/**
 * rfc792:
 *  The checksum is the 16-bit ones's complement of the one's
 *  complement sum of the ICMP message starting with the ICMP Type.
 *  For computing the checksum , the checksum field should be zero.
 *  This checksum may be replaced in the future.
 * 
 * One's complement sum: http://mathforum.org/library/drmath/view/54379.html
 * Apparently, not at all what's implemented in the github
 * 
 * Section 2.4.4.5.2 of https://tools.ietf.org/html/rfc1071 describes exactly
 * However, the protocol here is TCP, not ICMP
 */
impl ICMPPacket {
    // Checksum calculation: https://tools.ietf.org/html/rfc1071
    #[allow(dead_code)] // WTF rust
    fn new_not_working(typ: u8, code: u8, roh: u32) -> ICMPPacket {
        let mut checksum: u16 = (
            (((typ as u32) << 8 + (code as u32)) & (u16::max_value() as u32))
            + (((typ as u32) << 8 + (code as u32)) >> 16)
        ) as u16;
        // checksum field is 16b and should be ignored
        checksum = (
            ((((roh >> 16) as u32) + (checksum as u32)) & (u16::max_value() as u32))
            + ((((roh >> 16) as u32) + (checksum as u32)) >> 16)
        ) as u16;
        checksum = (
            (((roh & (u16::max_value() as u32)) + (checksum as u32)) & (u16::max_value() as u32))
            + (((roh & u16::max_value() as u32) + (checksum as u32)) >> 16)
        ) as u16;

        ICMPPacket{
            typ: typ,
            code: code,
            checksum: checksum,
            roh: roh
        }
    }

    #[allow(dead_code)] // WTF rust
    pub fn new(typ: u8, code: u8, roh: u32) -> ICMPPacket {
        let mut checksum = 0u16;
        
        checksum += typ as u16;
        checksum += code as u16;
        checksum += roh as u16;
        checksum += (roh << 8) as u16;
        checksum += (roh << 16) as u16;
        checksum += (roh << 24) as u16;
        ICMPPacket{
            typ: typ,
            code: code,
            checksum: checksum,
            roh: roh
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ICMPPacket;

    #[test]
    fn test_check_checksum() {
        let packet = ICMPPacket::new_not_working(8, 0, 0x13c20001);
        assert_eq!(packet.checksum, 0x4d71);
    }
}