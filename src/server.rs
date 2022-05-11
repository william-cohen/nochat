use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::error::Error;
use std::sync::mpsc;
use std::{thread, fs};

use crate::server::connection::Connection;
use crate::server::message::Message;

mod connection;
mod message;
mod pages;

pub struct Server {
    chatsubs: Vec<mpsc::Sender<Message>>,
    chatlog: Vec<Message>
}

impl Server {
    pub fn new() -> Self {
        Server {
            chatsubs: Vec::new(),
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
            connection::ClientAction::Join => self.handle_chat_stream(connection),
            connection::ClientAction::SendMessage => self.handle_message_post(connection),
            _ => self.handle_unknown_request(connection)
        }
    }

    fn handle_chat_stream(&mut self, mut connection: Connection) {
        let first = pages::render_chat_room();

        connection.push("HTTP/1.1 200 OK\r\nContent-type: text/html; charset=utf-8\r\n\r\n").unwrap();
        connection.push(&first).unwrap();
        for message in &self.chatlog {
            connection.push_message(message).unwrap();
        }

        let (tx, rx) = mpsc::channel::<Message>();

        self.chatsubs.push(tx);

        thread::spawn(move || {
            for message in rx.iter() {
                match connection.push_message(&message) {
                    Ok(_) => {},
                    Err(_error) => {
                        return
                    },
                }
            }
        });
    }

    fn handle_message_post(&mut self, connection: Connection) {
        if let Some(content) = connection.get_message() {
            let message = Message::new(&content);

            self.chatsubs = self.chatsubs
                .iter()
                .filter_map(|sender| sender
                    .send(message.clone())
                    .ok()
                    .map(|_| sender.clone())
                )
                .collect();

            self.chatlog.push(message);
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