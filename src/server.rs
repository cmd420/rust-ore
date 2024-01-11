use std::{error::Error, net::TcpListener, thread};

use crate::{client::Client, config::ServerConfig};

pub struct Server {
    listener: TcpListener,
    pub config: ServerConfig,
}

impl Server {
    /// Create a new server instance
    pub fn new(config: ServerConfig) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).unwrap();
        Self { listener, config }
    }

    /// Run the server
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Ok((stream, _)) = self.listener.accept() {
                let mut client = Client::new(stream);
                thread::spawn(move || match client.run() {
                    Ok(()) => (),
                    Err(err) => println!("Client error ocurred: {}", err),
                });
            }
        }
    }
}
