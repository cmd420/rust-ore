use std::{collections::HashMap, error::Error, path::Path};

use dot_properties_parser::{parse_properties_file, PropertyValue};

/// Minecraft 1.16.5 protocol version
pub const PROTOCOL_VERSION: i32 = 754;

/// Key value pairs containing the server configuration
pub struct ServerConfig(HashMap<String, PropertyValue>);

impl ServerConfig {
    // TODO: add better error handling than `Box<dyn Error>`
    /// Parses a `server.properties` file
    pub fn from_file(path: &str) -> Result<ServerConfig, Box<dyn Error>> {
        let path = Path::new(path);
        parse_properties_file(path, None).map(ServerConfig)
    }
}

impl std::ops::Deref for ServerConfig {
    type Target = HashMap<String, PropertyValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
