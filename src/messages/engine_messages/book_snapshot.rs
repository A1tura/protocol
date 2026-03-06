use bytes::{Buf, BufMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};


#[derive(Debug)]
pub struct BookSnapshot {
    pub order_id: u32,
    pub remaining: u32,
}

impl Message for BookSnapshot {
    const MSG_TYPE: u8 = 105;
    const MSG_SIZE: usize = 8;
}

impl Encode for BookSnapshot {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        todo!()
    }
}

impl Decode for BookSnapshot {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        todo!()
    }
}
