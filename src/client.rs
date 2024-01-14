use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use crate::{
    client_state::ClientState, config::PROTOCOL_VERSION, errors::ClientError, mc_packet::MCPacket,
    player::Player, prelude::ServerConfig, uuid::UUID,
};

pub struct Client {
    stream: TcpStream,
    state: ClientState,
    player: Player,
    config: Arc<ServerConfig>,
    is_running: bool,
}

impl Client {
    /// Creates a new instance
    pub fn new(stream: TcpStream, config: Arc<ServerConfig>) -> Self {
        Self {
            stream,
            state: ClientState::Handshake,
            player: Player::default(),
            config,
            is_running: false,
        }
    }

    /// Closes the connection between the server and client.
    /// Sets `is_running` to false, for now.
    pub fn close(&mut self) {
        self.is_running = false;
    }

    /// Runs the client instance
    pub fn run(&mut self) -> Result<(), ClientError> {
        if self.is_running {
            return Err(ClientError::AlreadyRunning);
        }

        self.is_running = true;
        let mut buffer = [0u8; 1024];
        loop {
            if !self.is_running {
                println!("Closing connection");
                return Ok(());
            }

            if let Ok(length) = self.stream.read(&mut buffer) {
                let packet = MCPacket::parse(buffer[..length].into())
                    .map_err(|err| ClientError::PacketError(err.to_string()))?;

                println!("Raw Packet: {:?}", &buffer[..length]);
                println!("Packet ID: {}", packet.id);
                println!("Packet Length: {}", packet.length);
                println!("Packet Data Length: {}", packet.data.len());
                println!("Total Packet Length: {}", length);

                match self.state {
                    ClientState::Handshake => self.handle_handshake(packet),
                    ClientState::Status => unimplemented!(),
                    ClientState::Login => self.handle_login(packet),
                    ClientState::Configuration => unimplemented!(),
                    ClientState::Play => unimplemented!(),
                }?;

                println!("End of packet\n");
            }
        }
    }

    /// Handle handshaking
    fn handle_handshake(&mut self, mut packet: MCPacket) -> Result<(), ClientError> {
        match packet.id {
            0x00 => {
                let prot_ver = packet.read_varint().expect("protocol version");
                let server_host = packet.read_string().expect("server host");
                let server_port = packet.read_unsigned_short().expect("server port");
                let next_state = packet.read_varint().expect("next state");

                if server_host != self.config.get("server-host").unwrap().as_ref() {
                    return Err(ClientError::IncorrectHost(server_host));
                }

                if prot_ver != PROTOCOL_VERSION {
                    return Err(ClientError::InvalidProtocolVersion(
                        prot_ver,
                        PROTOCOL_VERSION,
                    ));
                }

                println!("Protocol Version: {}", prot_ver);
                println!("Server Host: {}", server_host);
                println!("Server Port: {}", server_port);
                println!("Next State: {}", next_state);

                self.state = next_state.into();
            }
            _ => return Err(ClientError::InvalidPacketID(packet.id)),
        }

        Ok(())
    }

    /// Handle the Login process
    fn handle_login(&mut self, mut packet: MCPacket) -> Result<(), ClientError> {
        match packet.id {
            // Login Start
            0x00 => {
                self.player.username = packet.read_string().expect("username");
                if self
                    .config
                    .get("online-mode")
                    .unwrap_or(&true.into())
                    .value_as()
                {
                    unimplemented!()
                } else {
                    let uuid = UUID::new_rand();
                    let uuid_str = uuid.to_string();

                    println!("Player Username: {}", self.player.username);
                    println!("Generated UUID: {}", uuid_str);

                    let mut response = MCPacket::new(0x02);
                    response.write_uuid(uuid);
                    response.write_string(&self.player.username);

                    let response_bytes = response.finalize();
                    println!("Response Bytes: {:?}", response_bytes);

                    self.stream.write_all(&response_bytes).unwrap();
                    self.state = ClientState::Play;
                }
            }
            _ => return Err(ClientError::InvalidPacketID(packet.id)),
        }

        Ok(())
    }
}
