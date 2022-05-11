use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{ Read, Write, Error };
use std::slice::SliceIndex;
use regex::Regex;
use ammonia::clean;

use super::message::Message;


pub const CRLF: &str = "\r\n";


#[derive(Clone, Copy)]
pub enum ClientAction {
    Join,
    Login,
    SendMessage,
    Unknown
}

pub struct Connection {
    stream: TcpStream,
    action: ClientAction,
    request: String
}

impl Connection {
    pub fn new(mut stream: TcpStream) -> Self {
        let mut request_buf = [0; 1024];
        stream.read(&mut request_buf).unwrap();
        let request = String::from_utf8_lossy(&request_buf).to_string();
        println!("Request: {}", request);
        let re = Regex::new(r"(.*) ([^?]*)(?:\?message=)?(.*)? HTTP/1.1.*").unwrap();
        let captures = re.captures(&request);
        let action = captures
            .and_then(|matches| matches.get(1).zip(matches.get(2)))
            .map(|(verb_match, path_match)| {
                match (verb_match.as_str(), path_match.as_str()) {
                    ("GET", "/") => ClientAction::Join,
                    ("POST", "/chat") => ClientAction::SendMessage,
                    _ => ClientAction::Unknown
                }
            })
            .unwrap_or(ClientAction::Unknown);
        Connection { stream, action, request }
    }

    pub fn get_action(&self) -> ClientAction {
        self.action
    }

    pub fn get_message(&self) -> Option<String> {
        let re = Regex::new(r".*message=(.*)").unwrap();
        re
            .captures(&self.request)
            .and_then(|captures| captures.get(1))
            .map(|mat| mat.as_str().into())
            .and_then(|url_message| urlencoding::decode(url_message).ok())
            .map(|cow| cow.into_owned())
            .map(|message| message.replace("+", " "))
            .map(|message| clean(&message)) 
            
    }

    pub fn push(&mut self, content: &str) -> Result<(), Error> {
        self.stream
            .write_all(content.as_bytes())
            .and_then(|_| self.stream.flush())
    }

    pub fn push_message(&mut self, message: &Message) -> Result<(), Error> {
        self.push(&format!("<li><span>{}</span><a href=\"#\" class=\"btn active\">{}</a></li>", message.get_user(), message.get_content()))
    }
}