const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// Read a 32-bit integer
pub fn read_varint(data: &mut Vec<u8>) -> Option<i32> {
    let mut result = 0;
    let mut shift = 0;

    for byte in data.iter() {
        let value = *byte & !CONTINUE_BIT;
        result |= (value as i32) << shift;

        if *byte & CONTINUE_BIT == 0 {
            data.drain(0..shift / 7 + 1);
            return Some(result);
        }

        shift += 7;
    }

    None
}

/// Read a 64-bit integer
pub fn read_varlong(data: &mut Vec<u8>) -> Option<i64> {
    let mut result = 0;
    let mut shift = 0;

    for byte in data.iter() {
        let value = *byte & !CONTINUE_BIT;
        result |= (value as i64) << shift;

        if *byte & CONTINUE_BIT == 0 {
            data.drain(0..shift / 7 + 1);
            return Some(result);
        }

        shift += 7;

        if shift == 63 {
            let last_byte = *byte as i64;
            if last_byte & 0xFE != 0xFE {
                return None;
            }
        }
    }

    None
}

/// Write a 32-bit integer
pub fn write_varint(value: i32, packet: &mut Vec<u8>) {
    // let mut _value = value;
    // loop {
    //     if (_value & !SEGMENT_BITS as i32) == 0 {
    //         packet.push(_value as u8);
    //         break;
    //     }

    //     packet.push(((_value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8);
    //     _value >>= 7;
    // }
    let mut _value = value;

    loop {
        let mut byte = (_value & SEGMENT_BITS as i32) as u8;
        _value >>= 7;

        if _value != 0 {
            byte |= CONTINUE_BIT;
        }

        packet.push(byte);

        if _value == 0 {
            break;
        }
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

        packet.push(((_value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8);
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
