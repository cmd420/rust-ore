use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};

use crate::{client_state::ClientState, mc_packet::MCPacket, player::Player};

pub struct Client {
    stream: TcpStream,
    state: ClientState,
    player: Player,
}

impl Client {
    /// Creates a new instance
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: ClientState::Handshake,
            player: Player::default(),
        }
    }

    /// Runs the client instance
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0u8; 1024];
        while let Ok(length) = self.stream.read(&mut buffer) {
            let packet = MCPacket::parse(buffer[..length].into());

            println!("Raw Packet: {:?}", &buffer[..length]);
            println!("Packet ID: {}", packet.id);
            println!("Packet Length: {}", packet.length);
            println!("Packet Data Length: {}", packet.data.len());
            println!("Total Packet Length: {}", length);

            match self.state {
                ClientState::Handshake => self.handle_handshake(packet),
                ClientState::Status => unimplemented!(),
                ClientState::Login => self.handle_login(packet),
                ClientState::Configuration => {
                    unimplemented!()
                }
                ClientState::Play => unimplemented!(),
            }

            // self.stream.write_all(&buffer[..length])?;
            println!("End of packet\n");
        }

        println!("Socket closed.");
        Ok(())
    }

    /// Handle handshaking
    fn handle_handshake(&mut self, mut packet: MCPacket) {
        match packet.id {
            0x00 => {
                let prot_ver = packet.read_varint().expect("protocol version");
                let server_host = packet.read_string().expect("server host");
                let server_port = packet.read_unsigned_short().expect("server port");
                let next_state = packet.read_varint().expect("next state");

                println!("Protocol Version: {}", prot_ver);
                println!("Server Host: {}", server_host);
                println!("Server Port: {}", server_port);
                println!("Next State: {}", next_state);

                self.state = next_state.into();
            }
            _ => panic!("Invalid Packet ID!"),
        }
    }

    /// Handle the Login process
    fn handle_login(&mut self, mut packet: MCPacket) {
        match packet.id {
            // Login Start
            0x00 => {
                self.player.username = packet.read_string().expect("username");
                let uuid = uuid::Uuid::new_v3(&uuid::Uuid::NAMESPACE_DNS, self.player.username.as_bytes());

                println!("Player Username: {}", self.player.username);
                println!("Generated UUID: {}", uuid);

                let mut response = MCPacket::new(0x02);
                response.write_string(&uuid.to_string());
                response.write_string(&self.player.username);

                let response_bytes = response.finalize();
                println!("Response Bytes: {:?}", response_bytes);

                self.stream.write_all(&response_bytes).unwrap();
                self.state = ClientState::Play;
            }
            _ => panic!("Invalid Packet ID!"),
        }
    }
}
