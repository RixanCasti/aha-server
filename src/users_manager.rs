use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::user::User;

pub struct UsersManager {
    users_file_path: String,
}

impl UsersManager {
    pub fn new(users_file_path: &str) -> Self {
        if fs::metadata(users_file_path).is_err() {
            fs::File::create(users_file_path).unwrap();
        }

        Self {
            users_file_path: users_file_path.to_owned(),
        }
    }

    pub fn add_user(&self, user: &User) {
        let content = format!("{}:{}\n", user.get_id(), user.get_ip_address());

        let mut users_file = OpenOptions::new()
            .append(true)
            .open(&self.users_file_path)
            .unwrap();

        users_file.write_all(content.as_bytes()).unwrap();
    }

    pub fn compare_user(&self, user: &User) -> bool {
        self.get_users().contains(user)
    }

    fn get_users(&self) -> Vec<User> {
        unimplemented!()
    }
}
