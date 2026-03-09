use bytes::{Buf, BufMut};

use crate::{
    errors::ProtocolErrors,
    traits::{Decode, Encode, Message},
};

#[derive(Debug)]
pub struct Trade {
    pub maker_order_id: u32,
    pub taker_order_id: u32,

    pub price: f64,
    pub quantity: u32,
}

impl Message for Trade {
    const MSG_TYPE: u8 = 106;
    const MSG_SIZE: usize = 16;
}

impl Encode for Trade {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.maker_order_id);
        buf.put_u32(self.taker_order_id);

        buf.put_f64(self.price);
        buf.put_u32(self.quantity);
    }
}

impl Decode for Trade {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        return Ok(Self {
            maker_order_id: buf.get_u32(),

            taker_order_id: buf.get_u32(),

            price: buf.get_f64(),
            quantity: buf.get_u32(),
        });
    }
}
