use std::net::UdpSocket;

fn list_for_request(socket: &UdpSocket) {
    let error = false;
    while !error {
        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.resize(512, 0);

        println!("Receiving...");
        let (amt, _) = socket.recv_from(&mut buf).unwrap();
    }
}

pub fn start_server(host: String, port: u16) {
    println!("Binding UDP socket on: {}:{}", host, port);
    let socket: UdpSocket = UdpSocket::bind(format!("{}:{}", host, port)).unwrap();
    list_for_request(&socket);
}