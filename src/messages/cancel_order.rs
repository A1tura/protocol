use bytes::{Buf, BufMut, BytesMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};

#[derive(Debug)]
pub struct CancelOrder {
    pub symbol: u32,
    pub order_id: u32
}

impl Message for CancelOrder {
    const MSG_TYPE: u8 = 3;
    const MSG_SIZE: usize = 8;
}

impl Encode for CancelOrder {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol);
        buf.put_u32(self.order_id);
    }
}

impl Decode for CancelOrder {
    fn decode(buf: &mut BytesMut) -> Result<Self, ProtocolErrors> {
        if buf.len() != CancelOrder::MSG_SIZE { return Err(ProtocolErrors::InvalidMessageLength) };

        let symbol = buf.get_u32();
        let order_id = buf.get_u32();

        return Ok(Self {
            symbol,
            order_id
        })
    }
}
