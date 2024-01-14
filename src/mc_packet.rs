use crate::{errors::PacketError, util::*};

/// Uncompressed Minecraft packet consisting of:
/// - Packet length
/// - Packet id
/// - Packet data
pub struct MCPacket {
    pub length: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

impl MCPacket {
    /// Create a new packet for writing
    pub fn new(id: i32) -> Self {
        MCPacket {
            length: 0,
            id,
            data: vec![],
        }
    }

    /// Build uncompressed packet
    pub fn finalize(&mut self) -> Vec<u8> {
        // data length + 8 bytes (2 varints)
        let mut final_packet = Vec::<u8>::with_capacity(self.data.len() + 8);

        let mut id_bytes = Vec::<u8>::with_capacity(4);
        write_varint(self.id, &mut id_bytes);

        let total_length = self.data.len() + id_bytes.len();
        let mut length_bytes = Vec::<u8>::with_capacity(4);
        write_varint(total_length as i32, &mut length_bytes);

        final_packet.extend(length_bytes);
        final_packet.extend(id_bytes);
        final_packet.extend(&self.data);

        final_packet
    }

    /// Parse packet from bytes
    pub fn parse(mut bytes: Vec<u8>) -> Result<Self, PacketError> {
        let length = read_varint(&mut bytes)
            .ok_or_else(|| PacketError::ExpectedField("length".to_string()))?;
        let id = read_varint(&mut bytes)
            .ok_or_else(|| PacketError::ExpectedField("packet id".to_string()))?;

        Ok(MCPacket {
            length,
            id,
            data: bytes,
        })
    }

    /// Read a 32-bit integer
    pub fn read_varint(&mut self) -> Option<i32> {
        read_varint(&mut self.data)
    }

    /// Write a 32-bit integer
    pub fn write_varint(&mut self, value: i32) {
        write_varint(value, &mut self.data)
    }

    /// Read a 64-bit integer
    pub fn read_varlong(&mut self) -> Option<i64> {
        read_varlong(&mut self.data)
    }

    /// Write a 64-bit integer
    pub fn write_varlong(&mut self, value: i64) {
        write_varlong(value, &mut self.data)
    }

    /// Read a UTF-8 string
    pub fn read_string(&mut self) -> Option<String> {
        read_string(&mut self.data)
    }

    /// Write a UTF-8 string, prefixed with its length
    pub fn write_string(&mut self, value: &str) {
        write_string(value, &mut self.data)
    }

    /// Read an 16-bit unsigned short
    pub fn read_unsigned_short(&mut self) -> Option<u16> {
        read_u16(&mut self.data)
    }

    // TODO: implement write_unsigned_short

    // TODO: implement read_uuid

    /// Write UUID bytes
    pub fn write_uuid(&mut self, uuid: impl AsRef<[u8; 16]>) {
        self.data.extend_from_slice(uuid.as_ref());
    }
}
