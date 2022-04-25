use server::Server;

mod server;

fn main() {
    let mut server = Server::new();

    server.listen("127.0.0.1:7878").unwrap();
}

