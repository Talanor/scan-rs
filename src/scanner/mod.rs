use super::net::Host;

trait HostScanner {
    fn scan(Host) -> Self;
}

mod ping;

pub use self::ping::*;