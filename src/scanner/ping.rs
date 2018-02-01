use super::HostScanner;
use net::Host;

#[derive(Debug)]
pub struct PingScanner {
    version: String
}

impl HostScanner for PingScanner {
    fn scan(_host: Host) -> PingScanner {
        unimplemented!();
    }
}