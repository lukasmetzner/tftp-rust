mod client;
mod server;
use server::server::start_server;


fn main() {
    start_server(String::from("127.0.0.1"), 6969);
}
