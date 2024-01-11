use rust_ore::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        online_mode: false,
        ..Default::default()
    };
    let mut server = Server::new(config);

    server.run()?;
    Ok(())
}
