use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::error::Error;
use std::sync::mpsc;
use std::{thread, fs};
use pubsub::PubSub;

use crate::server::connection::Connection;

mod connection;

pub struct Server {
    pubsub: PubSub,
    chatlog: Vec<String>
}

impl Server {
    pub fn new() -> Self {
        Server {
            pubsub: PubSub::new(2),
            chatlog: Vec::new()
        }
    }

    pub fn listen<A: ToSocketAddrs>(&mut self , address: A) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            self.handle_tcpstream(stream?);
        }
        Ok(())
    }

    fn handle_tcpstream(&mut self, stream: TcpStream) {
        let connection = Connection::new(stream);

        match connection.get_action() {
            connection::ClientAction::READ_CHAT => self.handle_chat_stream(connection),
            connection::ClientAction::SEND_MESSAGE => self.handle_message_post(connection),
            _ => self.handle_unknown_request(connection)
        }
    }

    fn handle_chat_stream(&mut self, mut connection: Connection) {
        let first = fs::read_to_string("first.html").unwrap();

        connection.push("HTTP/1.1 200 OK\r\nContent-type: text/html; charset=utf-8\r\n\r\n").unwrap();
        connection.push(&first).unwrap();

        let sub = self.pubsub.lazy_subscribe("messages");

        for message in &self.chatlog {
            connection.push_message(message).unwrap();
        }

        thread::spawn(move || {
            let (tx, rx) = mpsc::channel::<String>();
            let sub = sub.activate(move |message| tx.send(message).unwrap());

            for message in rx.iter() {
                connection.push_message(&message).unwrap();
            }
            println!("Hang up: {:?}", sub);
        });
    }

    fn handle_message_post(&mut self, connection: Connection) {
        if let Some(message) = connection.get_message() {
            self.chatlog.push(message.clone());
            self.pubsub.notify("messages", &message);
            self.handle_chat_stream(connection);
        } else {
            self.handle_unknown_request(connection);
        }
    }

    fn handle_unknown_request(&self, mut connection: Connection) {
        connection.push("HTTP/1.1 400 Bad Request\r\n\r\n").unwrap();
        connection.push("<html><body><p>Invalid request</p></body></html>").unwrap();
    }

}