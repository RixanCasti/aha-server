use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::request::Request;
use crate::users_manager::UsersManager;

pub struct Server {
    listener: TcpListener,
    users_manager: UsersManager,
}

impl Server {
    pub fn new(port: &str, users_file_path: &str) -> Self {
        Self {
            listener: TcpListener::bind(format!("127.0.0.1:{port}")).unwrap(),
            users_manager: UsersManager::new(users_file_path),
        }
    }

    pub fn main(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let request = Request::from_connection(&stream);

            let (method, user) = request.unwrap();

            match method {
                "get" => {
                    let user_exists = self.users_manager.compare_user(user);
                    self.send_response(&stream, format!("{user_exists}").as_str());
                }
                "save" => {
                    let user_exists = self.users_manager.compare_user(user);

                    if user_exists {
                        self.send_response(&stream, "false");
                        return;
                    }

                    self.users_manager.add_user(user);
                    self.send_response(&stream, "true");
                }
                _ => {
                    eprintln!("Undefined request behavior. Aborting current connection.");
                    continue;
                }
            }
        }
    }

    fn send_response(&self, mut stream: &TcpStream, contents: &str) {
        let length = contents.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
