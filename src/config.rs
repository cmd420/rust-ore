// TODO: allow reading config from file
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub online_mode: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 25565,
            online_mode: true,
        }
    }
}
