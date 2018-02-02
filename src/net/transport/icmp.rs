/*
    Shamelessly overly-inspired from:
    https://github.com/MrKiven/rping/blob/master/src/main.rs

    Protocol RFC: https://tools.ietf.org/html/rfc792
*/

#[allow(dead_code)] // WTF rust??
#[derive(Debug)]
pub struct ICMPHeader {
    typ: u8,
    code: u8,
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
 * 
 * Checksum must be calculated with data in mind too:
 * https://www.scribd.com/doc/7074846/ICMP-and-Checksum-Calc
 */
impl ICMPHeader {
    #[allow(dead_code)] // WTF rust
    pub fn new(typ: u8, code: u8, roh: u32) -> ICMPHeader {
        ICMPHeader{
            typ: typ,
            code: code,
            roh: roh
        }
    }
}

#[derive(Debug)]
pub struct ICMPPacket {
    header: ICMPHeader,
    data: Vec<u8>
}

impl ICMPPacket {
    pub fn new(typ: u8, code: u8, roh: u32, data: Vec<u8>) -> ICMPPacket {
        ICMPPacket{
            header: ICMPHeader::new(
                typ,
                code,
                roh,
            ),
            data: data
        }
    }

    fn pack_for_checksum(&self) -> Vec<u16> {
        let mut data: Vec<u16> = Vec::new();

        data.push(
            ((self.header.typ as u16) << 8)
            + (self.header.code as u16)
        );
        data.push(0);
        data.push((self.header.roh >> 16) as u16);
        data.push((self.header.roh & 0b1111111111111111) as u16);

        // TODO: check if must pad or not in case of odd data
        let mut i = 0;
        while i < self.data.len() {
            data.push(
                ((self.data[i] as u16) << 8)
                + (self.data[i + 1] as u16)
            );
            i += 2;
        }
        data
    }

    pub fn checksum(&self) -> u16 {
        let mut checksum: u32 = 0;
        let data = self.pack_for_checksum();

        for item in data.into_iter() {
            print!("{:04X} ", item);
            let carry: u32 = (checksum + (item as u32)) >> 16;
            checksum = ((checksum + (item as u32)) & 0b1111111111111111) + carry;
        }

        checksum as u16
    }
}

#[cfg(test)]
mod tests {
    use super::ICMPPacket;

    #[test]
    fn test_check_checksum() {
        let data: Vec<u8> = vec![
            0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e,
            0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x61, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69
        ];
        let packet = ICMPPacket::new(
            8, 0, 0x02000900,
            data
        );
        assert_eq!(packet.checksum(), 0x425c);
    }
}