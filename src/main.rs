mod client;
mod server;
use server::server::TFTPServer;

fn main() {
    env_logger::init();
    let tftp_server = TFTPServer::new(
        String::from("./tftp-data"),
        String::from("127.0.0.1"), 
        6969
    );
    tftp_server.start_server();
}
