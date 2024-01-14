use std::{net::TcpListener, sync::Arc, thread};

use crate::{client::Client, config::ServerConfig, errors::ServerError};

pub struct Server {
    listener: TcpListener,
    pub config: Arc<ServerConfig>,
}

impl Server {
    /// Create a new server instance
    pub fn new(config: ServerConfig) -> Result<Self, ServerError> {
        let server = config
            .get("server-host")
            .ok_or_else(|| ServerError::MissingProperty("server-host".to_string()))?;

        let port = config
            .get("server-port")
            .ok_or_else(|| ServerError::MissingProperty("server-port".to_string()))?;

        let listener = TcpListener::bind(format!("{}:{}", server, port)).map_err(|err| {
            ServerError::SocketBindFailed(server.to_string(), port.value_as(), err.to_string())
        })?;

        Ok(Self {
            listener,
            config: Arc::new(config),
        })
    }

    /// Run the server
    pub fn run(&mut self) {
        loop {
            if let Ok((stream, _)) = self.listener.accept() {
                let config = self.config.clone();
                let mut client = Client::new(stream, config);
                thread::spawn(move || match client.run() {
                    Ok(()) => (),
                    Err(err) => eprintln!("Client error ocurred: {}", err),
                });
            }
        }
    }
}
