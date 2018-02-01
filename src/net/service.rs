#[derive(Debug)]
pub enum State {
    OPEN,
    CLOSED,
    FILTERED
}

#[derive(Debug)]
pub struct Service {
    name: String,
    port: u16,
    state: State
}

impl Service {
    pub fn new(name: &str, port: u16, state: State) -> Service {
        Service{
            name: name.to_string(),
            port: port,
            state: state
        }
    }
}