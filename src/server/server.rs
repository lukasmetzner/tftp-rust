use std::net::SocketAddr;
use std::{net::UdpSocket, fmt::Display};
use log::{debug, error, log_enabled, info, Level};
use std::fs::File;
use std::io::prelude::*;

pub struct TFTPServer {
    root_dir: String,
    socket: UdpSocket,
}

impl TFTPServer {
    /* 
    Error Codes
        Value     Meaning

        0         Not defined, see error message (if any).
        1         File not found.
        2         Access violation.
        3         Disk full or allocation exceeded.
        4         Illegal TFTP operation.
        5         Unknown transfer ID.
        6         File already exists.
        7         No such user.
     */
    fn send_error(&self, dest: SocketAddr, error_code: u16, error_msg: &String) {
        let mut error_packet: Vec<u8> = Vec::new();
        error_packet.append(&mut (5 as u16).to_be_bytes().to_vec());
        error_packet.append(&mut error_code.to_be_bytes().to_vec());
        error_packet.append(&mut error_msg.as_bytes().to_vec());
        self.socket.send_to(&error_packet.as_slice(), dest).unwrap();
    }

    fn handle_rrq(&self, buffer: &[u8; 512], dest: SocketAddr) {
        // Read after opcode to end of file name; terminated by zero byte
        let path: String = String::from_utf8(
            buffer[2..]
            .iter()
            .take_while(|x| {**x != 0x0})
            .map(|x| {*x})
            .collect()
        ).unwrap();

        // Try reading targeted file and send error on not found
        debug!("Received file path: {}", path);
        let mut file = {
            let this = File::open(path);
            match this {
                Ok(t) => t,
                Err(e) => {
                    self.send_error(dest, 1u16, &e.to_string());
                    return
                },
            }
        };
        let mut file_buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();
        debug!("Read {:?} bytes", file_buffer.len());
        let parts: usize = (file_buffer.len() as f32 / 498f32).ceil() as usize;
        debug!("Parts: {}", parts);

        let mut block: u16 = 1;
        let chunks: Vec<&[u8]> = file_buffer.chunks(parts).collect();
        for chunk in chunks {
            let mut tmp_buff: Vec<u8> = Vec::new();
            tmp_buff.append(&mut (3 as u16).to_be_bytes().to_vec());
            tmp_buff.append(&mut block.to_be_bytes().to_vec());
            tmp_buff.append(&mut chunk.to_vec());

            self.socket.send_to(&tmp_buff.as_slice(), &dest).unwrap();
            
            let mut recv_buffer: [u8; 512] = [0; 512];
            let (amt, _) = self.socket.recv_from(&mut recv_buffer).unwrap();
            let dest_opcode: u16 = u16::from_be_bytes([recv_buffer[0], recv_buffer[1]]);
            let dest_block: u16 = u16::from_be_bytes([recv_buffer[2], recv_buffer[3]]);

            if dest_opcode != 4 {
                //TODO: Handle not ACK package
            }

            if dest_block != block {
                //TODO: Handle wrong block -> resend
                break;
            }

            block += 1;
        }
    } 

    pub fn start_server(&self) {
        loop {
            let mut buffer: [u8; 512] = [0; 512];
            info!("Receiving!");
            let (amt, sock_addr) = self.socket.recv_from(&mut buffer).unwrap();
            debug!("Received {} bytes from {}", amt, sock_addr.to_string());
            // Switch between 5 different opcodes stored in the first 2 bytes
            match buffer[1] {
                1 => self.handle_rrq(&buffer, sock_addr),
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
