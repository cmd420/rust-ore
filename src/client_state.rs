#[repr(i32)]
pub enum ClientState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

impl From<i32> for ClientState {
    fn from(value: i32) -> Self {
        match value {
            0 => ClientState::Handshake,
            1 => ClientState::Status,
            2 => ClientState::Login,
            3 => ClientState::Configuration,
            4 => ClientState::Play,
            _ => panic!("Invalid value"),
        }
    }
}
