pub mod client;
pub mod client_state;
pub mod config;
pub mod entity;
pub mod errors;
pub mod mc_packet;
pub mod player;
pub mod server;
pub mod util;
pub mod uuid;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::client_state::ClientState;
    pub use crate::config::*;
    pub use crate::entity::Entity;
    pub use crate::errors::*;
    pub use crate::mc_packet::MCPacket;
    pub use crate::player::Player;
    pub use crate::server::Server;
}
