use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("another instance of client is already running")]
    AlreadyRunning,
    #[error("invalid packet id `{0}`")]
    InvalidPacketID(i32),
    #[error("packet contains an incorrect server host value `{0}`")]
    IncorrectHost(String),
    #[error("invalid protocol version `{0}`, expected `{1}`")]
    InvalidProtocolVersion(i32, i32),
    #[error("error parsing packet: {0}")]
    PacketError(String),
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("property `{0}` is missing")]
    MissingProperty(String),
    #[error("binding socket to host `{0}` port `{1}` failed: {2}")]
    SocketBindFailed(String, u16, String),
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("expected field `{0}`")]
    ExpectedField(String),
}
