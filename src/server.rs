/// Server struct
pub struct Server {
    addr: String,
}

impl Server {
    /// instantiates a new server object
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    /// runs a server object
    pub fn run(&self) {
        println!("Listening on {}", self.addr)
    }
}