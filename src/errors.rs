use bytes::{BufMut, BytesMut};

use super::header::HeaderError;

#[derive(Debug)]
pub enum ProtocolErrors {
    HeaderError(HeaderError),
    InvalidBody,
    SequenceError { expected: u32, received: u32 },
    InvalidMessageType,
    InvlaidMessageLength,
}

impl ProtocolErrors {
    pub fn get_error(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        match self {
            Self::HeaderError(err) => {
                match err {
                    HeaderError::InvalidLength => {
                        buf.put_u8(1);
                    },
                    HeaderError::InvalidHeader => {
                        buf.put_u8(2);
                    }
                }
            },
            Self::InvalidBody => buf.put_u8(3),
            Self::SequenceError { expected, received } => {
                buf.put_u8(4);
                buf.put_u32(*expected);
                buf.put_u32(*received);
            },
            Self::InvalidMessageType => buf.put_u8(5),
            Self::InvlaidMessageLength => buf.put_u8(6),
        }

        return buf;
    }
}

impl From<std::array::TryFromSliceError> for ProtocolErrors {
    fn from(_: std::array::TryFromSliceError) -> Self {
        Self::InvalidBody
    }
}
