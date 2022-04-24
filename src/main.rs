use server::Server;

mod server;

fn main() {
    let mut server = Server::new();

    server.listen("192.168.0.232:7878").unwrap();
}

