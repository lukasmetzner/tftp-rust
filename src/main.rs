use log::debug;
use simple_logger::SimpleLogger;
use std::{
    fs,
    net::UdpSocket,
    path::{Path, PathBuf}, thread::sleep, time::Duration,
};

pub struct Server {
    root_dir: PathBuf,
    udp_socket: UdpSocket,
}

impl Server {
    pub fn new(root_dir: String, address: String) -> Result<Server, anyhow::Error> {
        fs::create_dir_all(&root_dir)?;

        let udp_socket = UdpSocket::bind(&address)?;
        debug!("Bound udp socket to address {}", address);

        Ok(Server {
            root_dir: PathBuf::from(root_dir),
            udp_socket: udp_socket,
        })
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        loop {
            let mut buffer: [u8; 512] = [0; 512];
            let (_, addr) = self.udp_socket.recv_from(&mut buffer)?;
            debug!("Got datagram from client: {}", addr);
            self.process_client(buffer, addr)?;
        }
    }

    fn process_client(
        &self,
        init_packet: [u8; 512],
        address: std::net::SocketAddr,
    ) -> Result<(), anyhow::Error> {
        let opcode = u16::from_be_bytes([init_packet[0], init_packet[1]]);
        match opcode {
            1 => self.handle_rrq(init_packet, address),
            2 => self.handle_wrq(init_packet, address),
            _ => Ok(()),
        }
    }

    fn send_data_packet(
        &self,
        chunk: &[u8],
        block: u16,
        address: std::net::SocketAddr,
    ) -> Result<(), anyhow::Error> {
        let mut data_packet: Vec<u8> = Vec::new();
        data_packet.append((3 as u16).to_be_bytes().to_vec().as_mut());
        data_packet.append(block.to_be_bytes().to_vec().as_mut());
        data_packet.append(chunk.to_vec().as_mut());
        self.udp_socket.send_to(&data_packet, address)?;
        Ok(())
    }

    fn handle_rrq(
        &self,
        init_packet: [u8; 512],
        address: std::net::SocketAddr,
    ) -> Result<(), anyhow::Error> {
        let file_path = PathBuf::from(String::from_utf8(
            init_packet[2..]
                .iter()
                .take_while(|x| **x != 0)
                .map(|x| *x)
                .collect::<Vec<u8>>(),
        )?);
        debug!("Received filename {:?} from {}", file_path, address);
        let content = fs::read(self.root_dir.join(file_path))?;
        let chunks = content.chunks(512).collect::<Vec<&[u8]>>();
        let mut block: u16 = 1;
        for chunk in chunks {
            'acking: loop {
                self.send_data_packet(chunk, block, address)?;
                if self.validate_ack(block)? {
                    block += 1;
                    break 'acking;
                }
                sleep(Duration::from_secs(5));
            }
        }
        Ok(())
    }

    fn handle_wrq(
        &self,
        init_packet: [u8; 512],
        address: std::net::SocketAddr,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn validate_ack(&self, targeted_block: u16) -> Result<bool, anyhow::Error> {
        let mut recv_buffer: [u8; 512] = [0; 512];
        self.udp_socket.recv_from(&mut recv_buffer)?;

        let opcode: u16 = u16::from_be_bytes([recv_buffer[0], recv_buffer[1]]);
        if opcode != 4 {
            return Ok(false);
        }

        let received_block: u16 = u16::from_be_bytes([recv_buffer[2], recv_buffer[3]]);
        if received_block != targeted_block {
            return Ok(false);
        }

        Ok(true)
    }
}

fn main() -> Result<(), anyhow::Error> {
    SimpleLogger::new().env().init()?;
    let server = Server::new("./tftp-data".to_string(), "127.0.0.1:6969".to_string())?;
    server.run()?;
    Ok(())
}
