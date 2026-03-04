pub const HEADER_SIZE: usize = 9;

pub enum HeaderError {
    InvalidLength,
}

pub struct Header {
    pub version: u32,
    pub seq_num: u32,
    pub msg_type: u8,
}

impl Header {
    pub fn from(header: &[u8]) -> Result<Self, HeaderError> {
        if header.len() != HEADER_SIZE {
            return Err(HeaderError::InvalidLength);
        }

        let version = u32::from_le_bytes(header[..=4].try_into().unwrap());
        let seq_num = u32::from_le_bytes(header[4..=8].try_into().unwrap());
        let msg_type = u8::from_le_bytes(header[8..=9].try_into().unwrap());

        return Ok(Self { version, seq_num, msg_type });
    }
}
