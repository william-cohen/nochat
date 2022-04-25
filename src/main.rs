use server::Server;

mod server;

fn main() {
    let mut server = Server::new();
    server.listen("0.0.0.0:7878").unwrap();
}

