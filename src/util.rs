const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// Read a 32-bit integer
pub fn read_varint(data: &mut Vec<u8>) -> Option<i32> {
    let mut bytes = data.iter();
    let mut value = 0;
    let mut position = 0;
    let mut bytes_read: usize = 0;

    loop {
        let curr_byte = bytes.next().unwrap();
        bytes_read += 1;
        value |= ((curr_byte & SEGMENT_BITS) << position) as i32;

        if curr_byte & CONTINUE_BIT == 0 {
            break;
        }

        position += 7;

        if position > 32 {
            panic!("VarInt is too big!");
        }
    }

    *data = data.split_off(bytes_read);
    Some(value)
}

/// Read a 64-bit integer
pub fn read_varlong(data: &mut Vec<u8>) -> Option<i64> {
    let mut bytes = data.iter();
    let mut value = 0;
    let mut position = 0;
    let mut bytes_read: usize = 0;

    loop {
        let curr_byte = bytes.next().unwrap();
        bytes_read += 1;
        value |= ((curr_byte & SEGMENT_BITS) << position) as i64;

        if curr_byte & CONTINUE_BIT == 0 {
            break;
        }

        position += 7;

        if position > 64 {
            panic!("VarLong is too big!");
        }
    }

    *data = data.split_off(bytes_read);
    Some(value)
}

/// Write a 32-bit integer
pub fn write_varint(value: i32, packet: &mut Vec<u8>) {
    let mut _value = value;
    loop {
        if (_value & !SEGMENT_BITS as i32) == 0 {
            packet.push(_value as u8);
            break;
        }

        packet.push(((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8);
        _value >>= 7;
    }
}

/// Write a 64-bit integer
pub fn write_varlong(value: i64, packet: &mut Vec<u8>) {
    let mut _value = value;
    loop {
        if (_value & !SEGMENT_BITS as i64) == 0 {
            packet.push(_value as u8);
            break;
        }

        packet.push(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8);
        _value >>= 7;
    }
}

/// Read a UTF-8 string
pub fn read_string(data: &mut Vec<u8>) -> Option<String> {
    let str_len = read_varint(data);
    if let Some(str_len) = str_len {
        let str_bytes = data.drain(0..str_len as usize).collect();

        return match String::from_utf8(str_bytes) {
            Ok(string) => Some(string),
            Err(_) => None,
        };
    }

    None
}

/// Write a UTF-8 string, prefixed with its length
pub fn write_string(value: &str, packet: &mut Vec<u8>) {
    let bytes = value.as_bytes();
    write_varint(bytes.len() as i32, packet);
    packet.extend_from_slice(bytes);
}

/// Read a 16-bit unsigned short
pub fn read_u16(data: &mut Vec<u8>) -> Option<u16> {
    if data.len() < 2 {
        return None;
    }

    let value = u16::from_le_bytes([data[0], data[1]]);
    data.drain(0..2);

    Some(value)
}
