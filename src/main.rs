mod request;
mod server;
mod user;
mod users_manager;

use server::Server;

const PORT: &str = "7878";

fn main() {
    let server = Server::new(PORT, "users.aha");
    server.main();
}
