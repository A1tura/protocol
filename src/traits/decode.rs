use bytes::BytesMut;

use crate::errors::ProtocolErrors;

pub trait Decode: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, ProtocolErrors>;
}
