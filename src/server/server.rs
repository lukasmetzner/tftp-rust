use std::{net::UdpSocket, fmt::Display};
use log::{debug, error, log_enabled, info, Level};

#[repr(u16)]
enum Opcode {
    Rrq=1,
    Wrq=2,
    Data=3,
    Ack=4,
    Error=5,
}

impl Opcode {
    pub fn from_u16(value: u16) -> Opcode {
        match value {
            1 => Opcode::Rrq,
            2 => Opcode::Wrq,
            3 => Opcode::Data,
            4 => Opcode::Ack,
            5 => Opcode::Error,
            _ => panic!("Unkown opcode!"),
        }
    }
}

pub struct TFTPServer {
    root_dir: String,
    socket: UdpSocket,
}

impl TFTPServer {
    fn handle_rrq(&self, buffer: &[u8; 512]) {
        let data: Vec<u8> = 
            buffer[2..]
            .iter()
            .filter_map(|x| {
                if *x == 0x0 {
                    None
                } else {
                    Some(*x)
                }
            })
            .collect();
    } 

    pub fn start_server(&self) {
        loop {
            let mut buffer: [u8; 512] = [0; 512];
            info!("Receiving!");
            let (amt, sock_addr) = self.socket.recv_from(&mut buffer).unwrap();
            debug!("Received {} bytes from {}", amt, sock_addr.to_string());
            let opcode: Opcode = Opcode::from_u16(u16::from_be_bytes([buffer[0], buffer[1]]));
            match opcode as Opcode {
                Opcode::Rrq => self.handle_rrq(&buffer),
                Opcode::Wrq => {debug!("Wrq")},
                Opcode::Data => {debug!("Data")},
                Opcode::Ack => {debug!("Ack")},
                Opcode::Error => {debug!("Error")},
            }
        }
    }

    pub fn new(root_dir: String, host: String, port: u16) -> Self {
        info!("Binding UDP socket on: {}:{}", host, port);
        Self { root_dir: root_dir, socket: UdpSocket::bind(format!("{}:{}", host, port)).unwrap() } 
    }
}
