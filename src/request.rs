use std::io::BufRead;
use std::{io::BufReader, net::TcpStream};

use crate::user::User;

pub struct Request {
    method: String,
    user: User,
}

impl Request {
    pub fn from_connection(mut stream: &TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);

        // Before: GET /?method=get&id=hermione&ip=1.1.1.1 HTTP/1.1
        let request = buf_reader.lines().next().unwrap().unwrap_or("".to_owned());
        let request = request.split_whitespace().nth(1).unwrap();

        if !(request.matches('&').count() == 2) {
            return Self {
                method: "".to_owned(),
                user: User::new("".to_owned(), "".to_owned()),
            };
        }

        let request: Vec<&str> = request.split('&').collect();
        let request = (
            request[0].split('=').collect::<Vec<&str>>()[1],
            request[1].split('=').collect::<Vec<&str>>()[1],
            request[2].split('=').collect::<Vec<&str>>()[1],
        );
        // After: (get, hermione, 1.1.1.1)

        let method = match request.0.find("get") {
            Some(_) => "get",
            None => match request.0.find("save") {
                Some(_) => "save",
                None => "",
            },
        };

        let id = request.1.to_owned();
        let ip_address = request.2.to_owned();

        Self {
            method: method.to_owned(),
            user: User::new(id, ip_address),
        }
    }

    pub fn unwrap(&self) -> (&str, &User) {
        (&self.method, &self.user)
    }
}
