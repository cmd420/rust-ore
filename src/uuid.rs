use rand::prelude::*;

/// Universally Unique Identifier struct
/// Stores the most and least significant bits
pub struct UUID {
    msb: i64,
    lsb: i64,
    bytes: [u8; 16]
}

impl UUID {
    /// Generates random bytes to then be used to construct a new UUID
    pub fn new_rand() -> Self {
        let mut bytes = [0u8; 16];
        thread_rng().fill(&mut bytes);

        Self::from_bytes(&bytes)
    }

    /// Constructs a new UUID from the MD5 hash of `name`
    pub fn from_name(name: &str) -> Self {
        let bytes = md5::compute(name);
        Self::from_bytes(&bytes)
    }

    /// Constructs a new UUID from a byte array
    pub fn from_bytes(data: &[u8; 16]) -> Self {
        let mut msb = 0i64;
        let mut lsb = 0i64;

        for byte in data.iter().take(8) {
            msb = (msb << 8) | *byte as i64;
        }

        for byte in data.iter().take(16).skip(8) {
            lsb = (lsb << 8) | *byte as i64
        }

        Self { msb, lsb, bytes: *data }
    }

    // pub fn version(&self) -> i64 {
    //     self.msb >> 12 & 0x0F
    // }

    // pub fn variant(&self) -> i64 {
    //     (self.lsb >> (64 - (self.lsb >> 62))) & (self.lsb >> 63)
    // }

    fn digits(val: i64, digits: usize) -> String {
        let hi: i64 = 1 << (digits * 4);
        format!("{:x}", hi | (val & (hi - 1)))[1..].to_string()
    }
}

impl ToString for UUID {
    /// Converts the UUID bytes to a UUID string
    /// with a format of:
    /// xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    fn to_string(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            Self::digits(self.msb >> 32, 8),
            Self::digits(self.msb >> 16, 4),
            Self::digits(self.msb, 4),
            Self::digits(self.lsb >> 48, 4),
            Self::digits(self.lsb, 12)
        )
    }
}


impl AsRef<[u8; 16]> for UUID {
    fn as_ref(&self) -> &[u8; 16] {
        &self.bytes
    }
}