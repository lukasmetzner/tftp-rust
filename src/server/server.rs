use std::{net::UdpSocket, fmt::Display};
use log::{debug, error, log_enabled, info, Level};
use std::fs::File;
use std::io::prelude::*;

pub struct TFTPServer {
    root_dir: String,
    socket: UdpSocket,
}

impl TFTPServer {
    fn handle_rrq(&self, buffer: &[u8; 512]) {
        // Read after opcode to end of file name; terminated by zero byte
        let path: String = String::from_utf8(
            buffer[2..]
            .iter()
            .take_while(|x| {**x != 0x0})
            .map(|x| {*x})
            .collect()
        ).unwrap();

        // Try reading targeted file
        debug!("Received file path: {}", path);
        let mut file = File::open(path).unwrap();
        let mut file_buffer: Vec<u8> = Vec::new();
        // TODO: Send error on not found
        file.read_to_end(&mut file_buffer).unwrap();
        debug!("Read {:?} bytes", file_buffer.len());
        let parts: f32 = (file_buffer.len() as f32 / 498f32).ceil();
        debug!("Parts: {}", parts);
    } 

    pub fn start_server(&self) {
        loop {
            let mut buffer: [u8; 512] = [0; 512];
            info!("Receiving!");
            let (amt, sock_addr) = self.socket.recv_from(&mut buffer).unwrap();
            debug!("Received {} bytes from {}", amt, sock_addr.to_string());
            // Switch between 5 different opcodes stored in the first 2 bytes
            match buffer[1] {
                1 => self.handle_rrq(&buffer),
                2 => {debug!("Wrq")},
                3 => {debug!("Data")},
                4 => {debug!("Ack")},
                5 => {debug!("Error")},
                _ => panic!("Unkown opcode!"),
            }
        }
    }

    pub fn new(root_dir: String, host: String, port: u16) -> Self {
        info!("Binding UDP socket on: {}:{}", host, port);
        Self { root_dir: root_dir, socket: UdpSocket::bind(format!("{}:{}", host, port)).unwrap() } 
    }
}
