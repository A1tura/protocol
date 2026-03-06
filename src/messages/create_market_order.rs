use bytes::{Buf, BufMut, BytesMut};

use crate::traits::{Encode, Message, Decode};

#[derive(Debug)]
pub struct CreateMarketOrder {
    pub symbol: u32,
    pub side: u8,
    pub quantity: u32,
}

impl Message for CreateMarketOrder {
    const MSG_TYPE: u8 = 2;
}

impl Encode for CreateMarketOrder {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol);
        buf.put_u8(self.side);
        buf.put_u32(self.quantity);
    }
}

impl Decode for CreateMarketOrder {
    fn decode(buf: &mut BytesMut) -> Self {
        let symbol = buf.get_u32();
        let side = buf.get_u8();
        let quantity = buf.get_u32();

        return Self {
            symbol,
            side,
            quantity
        }
    }
}
