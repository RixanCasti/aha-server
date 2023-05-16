mod request;
mod server;
mod user;

use server::Server;

const PORT: &str = "7878";

fn main() {
    let server = Server::new(PORT);
    server.main();
}
