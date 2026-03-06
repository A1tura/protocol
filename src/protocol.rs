use bytes::{Bytes, BytesMut};

use crate::{
    errors::{self, ProtocolErrors},
    messages,
    traits::Decode,
};

#[derive(Debug)]
pub enum Message {
    CreateLimitOrder(messages::CreateLimitOrder),
    CreateMarketOrder(messages::CreateMarketOrder),
    CancelOrder(messages::CancelOrder),
    Error(errors::ProtocolErrors),
}

#[derive(Debug)]
pub enum MessageType {
    CreateLimitOrder = 1,
    CreateMarketOrder = 2,
    CancelOrder = 3,
    Error = 4,
}

impl TryFrom<u8> for MessageType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, String> {
        match value {
            1 => Ok(Self::CreateLimitOrder),
            2 => Ok(Self::CreateMarketOrder),
            3 => Ok(Self::CancelOrder),
            4 => Ok(Self::Error),
            _ => Err(String::from("Invalid value")),
        }
    }
}

impl Message {
    pub fn decode(msg_type: u8, body: &mut BytesMut) -> Result<Self, ProtocolErrors> {
        match msg_type {
            1 => return Ok(Message::CreateLimitOrder(messages::CreateLimitOrder::decode(body))),
            2 => return Ok(Message::CreateMarketOrder(messages::CreateMarketOrder::decode(body))),
            3 => return Ok(Message::CancelOrder(messages::CancelOrder::decode(body))),
            _ => return Err(ProtocolErrors::InvalidMessageType),
        }
    }
}
