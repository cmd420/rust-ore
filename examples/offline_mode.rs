use rust_ore::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::from_file("examples/common/offline.server.properties")?;
    Server::new(config)?.run();
    
    Ok(())
}
