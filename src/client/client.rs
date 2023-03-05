use std::net::UdpSocket;
use operations::{Data, Rrq, OPCODE, send_ack};

use super::operations;

fn send_u16_buffer(socket: &UdpSocket, buffer: &Vec<u16>) -> std::io::Result<usize> {
    let transformed_buffer: Vec<u8> = buffer.iter().map(|f| {f.to_be_bytes()}).flatten().collect();
    socket.send_to(&transformed_buffer, "127.0.0.1:6969")
}

fn tftp_recv_data(socket: &UdpSocket) -> Vec<Data> {
    let mut block: u16 = 1;
    let mut last_length: usize = 512;
    let mut fullbuffer: Vec<Data> = Vec::new();
    while last_length == 512 {

        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.resize(512, 0);

        let (amt, _) = socket.recv_from(&mut buf).unwrap();
        let data: Data = Data::from_buffer(&buf);

        send_ack(&socket, block).unwrap();

        block = data.block + 1;
        last_length = amt;

        fullbuffer.push(data);
    }
    fullbuffer
}

fn recv_file(socket: &UdpSocket, filename: &str) -> Vec<Data> {
    let rrq = Rrq {
        opcode: OPCODE::RRW as u16,
        filename: filename.to_string(),
        mode: "octet".to_string(),
    };
    socket.send_to(&rrq.to_buffer(), "127.0.0.1:6969").unwrap();
    tftp_recv_data(&socket)
}

pub fn client_test() {
    println!("Bind socket!");
    let socket = UdpSocket::bind("127.0.0.1:6970").unwrap();
    let buffer = recv_file(&socket, "loremipsum.txt");
    for data in buffer {
        println!("{:?}", String::from_utf8(data.data).unwrap());
    }
}