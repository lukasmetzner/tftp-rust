use std::net::UdpSocket;

pub enum OPCODE {
    RRW = 1,
    WRQ = 2,
    DATA = 3,
    ACK = 4,
    ERROR = 5,
}

pub fn send_ack(socket: &UdpSocket, block: u16) -> std::io::Result::<usize> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut (OPCODE::ACK as u16).to_be_bytes().to_vec());
    buffer.append(&mut block.to_be_bytes().to_vec());
    socket.send_to(&buffer, "127.0.0.1:6969")
}

pub struct Rrq {
    pub opcode: u16,
    pub filename: String,
    pub mode: String,
}

impl Rrq {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.append(&mut self.opcode.to_be_bytes().to_vec());
        buffer.append(&mut self.filename.as_bytes().to_vec());
        buffer.push(0);
        buffer.append(&mut self.mode.as_bytes().to_vec());
        buffer.push(0);
        buffer
    }
}

pub struct Data {
    pub opcode: u16,
    pub block: u16,
    pub data: Vec<u8>,
}

impl Data {
    pub fn from_buffer(buffer: &Vec<u8>) -> Data {
        let opcode = u16::from_be_bytes([buffer[0], buffer[1]]);
        let block = u16::from_be_bytes([buffer[2], buffer[3]]);
        let data = 
            buffer[4..]
            .iter()
            .rev()
            .filter_map(|x| {
                if *x == 0x0 {
                    None
                } else {
                    Some(*x)
                }})
            .collect();
        Data {
            opcode,
            block,
            data,
        }
    }
}