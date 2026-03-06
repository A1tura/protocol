use bytes::{Buf, BufMut, BytesMut};

use crate::traits::{Decode, Encode, Message};

#[derive(Debug)]
pub struct CancelOrder {
    symbol: u32,
    order_id: u32
}

impl Message for CancelOrder {
    const MSG_TYPE: u8 = 3;
}

impl Encode for CancelOrder {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol);
        buf.put_u32(self.order_id);
    }
}

impl Decode for CancelOrder {
    fn decode(buf: &mut BytesMut) -> Self {
        let symbol = buf.get_u32();
        let order_id = buf.get_u32();

        return Self {
            symbol,
            order_id
        }
    }
}
