use std::{collections::HashMap, error::Error, path::Path};

use dot_properties_parser::{parse_properties_file, PropertyValue};

/// Minecraft 1.16.5 protocol version
pub const PROTOCOL_VERSION: i32 = 754;

pub struct ServerConfig(HashMap<String, PropertyValue>);

impl ServerConfig {
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
