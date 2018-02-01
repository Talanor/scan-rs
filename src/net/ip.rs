use std;
extern crate regex;

pub trait IP {
    fn new(&str) -> Self;
    fn get_cidr(&self) -> String;
    fn get_host(&self) -> String;
    fn get_net_mask(&self) -> String;
}

#[derive(Debug)]
pub struct IPv4 {
    host:       std::net::Ipv4Addr,
    net_mask:   u32
}


#[derive(Debug)]
pub enum IPAddr {
    V4(IPv4)
}

impl IPv4 {
    pub fn get_net_mask_length(&self) -> u8 {
        let mut mask: u32 = 0;
        let mut ret: u8 = 32;

        for n in 0..32 {
            if (self.net_mask ^ mask) == (self.net_mask | mask) {
                ret = n;
                mask = (mask << 1) | 1;
            } else {
                break
            }
        }
        32 - ret
    }

    pub fn to_ipv4addrs(&self) -> Vec<std::net::Ipv4Addr> {
        let mut addrs: Vec<std::net::Ipv4Addr> = vec![];
        let host = u32::from(self.host);
        let mut ip = host & self.net_mask;

        while (ip & self.net_mask) == (host & self.net_mask) {
            addrs.push(std::net::Ipv4Addr::new(
                (ip / (256 * 256 * 256)) as u8,
                (ip / (256 * 256) % 256) as u8,
                ((ip / 256) % 256) as u8,
                (ip % 256) as u8
            ));
            ip = ip + 1;
        }
        addrs
    }
}

impl IP for IPv4 {
    fn new(descr: &str) -> IPv4 {
        let re = regex::Regex::new(r"(\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})/(\d{1,2})").unwrap();
        let caps = re.captures(descr).unwrap();

        let mut net_mask: u32 = u32::max_value();

        for _ in caps.get(5).unwrap().as_str().parse::<u32>().unwrap()..32 {
            net_mask = net_mask << 1;
        }

        IPv4 {
            host: std::net::Ipv4Addr::new(
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap() as u8,
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap() as u8,
                caps.get(3).unwrap().as_str().parse::<u32>().unwrap() as u8,
                caps.get(4).unwrap().as_str().parse::<u32>().unwrap() as u8
            ),
            net_mask: net_mask
        }
    }

    fn get_cidr(&self) -> String {
        format!("{}/{}", self.get_host(), self.get_net_mask_length())
    }

    fn get_host(&self) -> String {
        let octets = self.host.octets();
        format!(
            "{}.{}.{}.{}",
            octets[0],
            octets[1],
            octets[2],
            octets[3]
        )
    }

    fn get_net_mask(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.net_mask / (256 * 256 * 256),
            (self.net_mask / (256 * 256)) % 256,
            (self.net_mask / 256) % 256,
            self.net_mask % 256
        )
    }
}

#[cfg(test)]
mod tests {
    use std;
    use super::IPv4;
    use super::IP;
    
    #[test]
    fn test_get_host() {
        assert_eq!(
            IPv4::new("192.168.1.0/24").get_host(),
            "192.168.1.0"
        );
    }

    #[test]
    fn test_get_net_mask() {
        assert_eq!(
            IPv4::new("192.168.1.0/24").get_net_mask(),
            "255.255.255.0"
        );
    }

    #[test]
    fn test_get_cidr() {
        assert_eq!(
            IPv4::new("192.168.1.0/24").get_cidr(),
            "192.168.1.0/24"
        );
    }

    #[test]
    fn test_get_addresses() {
        let mut octets = [192, 168, 1, 0];
        let fmt = format!(
            "{}.{}.{}.{}/24",
            octets[0], octets[1], octets[2], octets[3]
        );
        let addresses = IPv4::new(&fmt).to_ipv4addrs();

        for ip in addresses {
            assert_eq!(octets, ip.octets());
            let tip = std::net::Ipv4Addr::new(
                octets[0], octets[1], octets[2], octets[3]
            );
            octets = std::net::Ipv4Addr::from(u32::from(tip) + 1).octets();
        }
    }
}