use rust_ore::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        online_mode: false,
        ..Default::default()
    };

    Server::new(config).run()
}
