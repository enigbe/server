use std::{net::TcpListener, io::Read, convert::{TryFrom, TryInto}};
use crate::http::Request;


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
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();
            match res {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                    println!("Address: {:?}", addr);
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e)
                },
            }
        }
    }
}