#[derive(PartialEq)]
pub struct User {
    id: String,
    ip_address: String,
}

impl User {
    pub fn new(id: String, ip_address: String) -> Self {
        Self { id, ip_address }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_ip_address(&self) -> &str {
        &self.ip_address
    }
}
