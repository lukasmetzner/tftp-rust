use std::net::UdpSocket;
use std::io::{self, BufRead};

enum OPCODE {
    RRW = 1,
    WRQ = 2,
    DATA = 3,
    ACK = 4,
    ERROR = 5,
}

fn send_u16_buffer(socket: &UdpSocket, buffer: &Vec<u16>) -> std::io::Result<usize> {
    let transformed_buffer: Vec<u8> = buffer.iter().map(|f| {f.to_be_bytes()}).flatten().collect();
    socket.send_to(&transformed_buffer, "127.0.0.1:6969")
}

fn send_ack(socket: &UdpSocket, block: u16) -> std::io::Result<()> {
    let mut buffer: Vec<u16> = Vec::new();
    buffer.push(OPCODE::ACK as u16);
    buffer.push(block);
    send_u16_buffer(&socket, &buffer).unwrap();
    Ok(())
}

fn tftp_recv_data(socket: &UdpSocket) -> Vec<DATA> {
    let mut block: u16 = 1;
    let mut last_length: usize = 512;
    let mut fullbuffer: Vec<DATA> = Vec::new();
    while last_length == 512 {

        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.resize(512, 0);

        let (amt, _) = socket.recv_from(&mut buf).unwrap();
        let data: DATA = DATA::from_buffer(&buf);

        send_ack(&socket, block).unwrap();

        block = data.block + 1;
        last_length = amt;

        fullbuffer.push(data);
    }
    fullbuffer
}

struct RRQ {
    opcode: u16,
    filename: String,
    mode: String,
}

impl RRQ {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.append(&mut self.opcode.to_be_bytes().to_vec());
        buffer.append(&mut self.filename.as_bytes().to_vec());
        buffer.push(0);
        buffer.append(&mut self.mode.as_bytes().to_vec());
        buffer.push(0);
        buffer
    }
}

struct DATA {
    opcode: u16,
    block: u16,
    data: Vec<u8>,
}

impl DATA {
    fn from_buffer(buffer: &Vec<u8>) -> DATA {
        let opcode = u16::from_be_bytes([buffer[0], buffer[1]]);
        let block = u16::from_be_bytes([buffer[2], buffer[3]]);
        let data = buffer[4..].to_vec();
        DATA {
            opcode,
            block,
            data,
        }
    }
}

fn recv_file(socket: &UdpSocket, filename: &str) -> Vec<DATA> {
    let rrq = RRQ {
        opcode: OPCODE::RRW as u16,
        filename: filename.to_string(),
        mode: "octet".to_string(),
    };
    socket.send_to(&rrq.to_bytes(), "127.0.0.1:6969").unwrap();
    tftp_recv_data(&socket)
}

fn main() {
    println!("Bind socket!");
    let socket = UdpSocket::bind("127.0.0.1:6970").unwrap();
    let buffer = recv_file(&socket, "loremipsum.txt");
    for data in buffer {
        println!("{:?}", String::from_utf8(data.data).unwrap());
        println!("{:?}", data.block);
        println!("{:?}", data.opcode);
    }
}
