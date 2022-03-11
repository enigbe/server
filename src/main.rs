mod server;
mod http;

use server::Server;

fn main() {
    // 1. Create an address containing host and port information
    let addr = String::from("127.0.0.1:8080");
    // 2. Create a new server object
    let server = Server::new(addr);

    // 3. Run server
    server.run();
}
