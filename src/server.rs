use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::request::Request;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(port: &str) -> Self {
        Self {
            listener: TcpListener::bind(format!("127.0.0.1:{port}")).unwrap(),
        }
    }

    pub fn main(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let request = Request::from_connection(&stream);

            let (method, _user) = request.unwrap();

            match method {
                "get" => (),
                "save" => (),
                _ => {
                    eprintln!("Undefined request behavior. Aborting current connection.");
                    continue;
                }
            }
        }
    }

    fn _send_response(&self, mut stream: &TcpStream, contents: &str) {
        let length = contents.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
