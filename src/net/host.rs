use net::ip;
use net::service;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Host {
    addr: ip::IPAddr,
    services: Vec<service::Service>,
    attrs: HashMap<String, String>
}