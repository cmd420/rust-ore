use thiserror::Error;

/// Errors caused by the Minecraft client
#[derive(Error, Debug)]
pub enum ClientError {
    /// Excluding this one
    #[error("another instance of client is already running")]
    AlreadyRunning,
    /// Client provided an invalid packet ID
    #[error("invalid packet id `{0}`")]
    InvalidPacketID(i32),
    /// Client provided a server host different than the one configured in server.properties
    #[error("packet contains an incorrect server host value `{0}`")]
    IncorrectHost(String),
    /// Client is on a different Minecraft version
    #[error("invalid protocol version `{0}`, expected `{1}`")]
    InvalidProtocolVersion(i32, i32),
    /// Packet parsing error, for now
    #[error("error parsing packet: {0}")]
    PacketError(String),
}

/// Errors caused by the server
#[derive(Error, Debug)]
pub enum ServerError {
    /// Required property in server configuration is missing
    #[error("property `{0}` is missing")]
    MissingProperty(String),
    /// Binding to socket failed
    #[error("binding socket to host `{0}` port `{1}` failed: {2}")]
    SocketBindFailed(String, u16, String),
}

/// Errors caused by Minecraft packets
#[derive(Error, Debug)]
pub enum PacketError {
    /// Expected a field of a certain type
    #[error("expected field `{0}`")]
    ExpectedField(String),
}
