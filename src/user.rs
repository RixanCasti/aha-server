pub struct User {
    id: String,
    ip_address: String,
}

impl User {
    pub fn new(id: String, ip_address: String) -> Self {
        Self { id, ip_address }
    }
}
