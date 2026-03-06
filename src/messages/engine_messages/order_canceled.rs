use bytes::{Buf, BufMut};

use crate::{
    errors::ProtocolErrors,
    traits::{Decode, Encode, Message},
};

#[derive(Debug)]
pub struct OrderCanceled {
    pub order_id: u32,
}

impl Message for OrderCanceled {
    const MSG_TYPE: u8 = 102;
    const MSG_SIZE: usize = 4;
}

impl Encode for OrderCanceled {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.order_id);
    }
}

impl Decode for OrderCanceled {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        return Ok(Self {
            order_id: buf.get_u32(),
        });
    }
}
