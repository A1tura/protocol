use bytes::BytesMut;

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

    OrderAccepted(messages::engine_messages::OrderAccepted),
    OrderCanceled(messages::engine_messages::OrderCanceled),
    OrderPartiallyFilled(messages::engine_messages::OrderPartiallyFilled),
    OrderFilled(messages::engine_messages::OrderFilled),
    BookSnapshot(messages::engine_messages::BookSnapshot),
    Trade(messages::engine_messages::Trade),
}

#[derive(Debug)]
pub enum MessageType {
    // 1 - 101 User Messages
    CreateLimitOrder = 1,
    CreateMarketOrder = 2,
    CancelOrder = 3,
    Error = 4,

    // 101 - 201 - Engine Messages
    OrderAccepted = 101,
    OrderCanceled = 102,
    OrderPartiallyFilled = 103,
    OrderFilled = 104,
    BookSnapshot = 105,
    Trade = 106,
}

impl TryFrom<u8> for MessageType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, String> {
        match value {
            1 => Ok(Self::CreateLimitOrder),
            2 => Ok(Self::CreateMarketOrder),
            3 => Ok(Self::CancelOrder),
            4 => Ok(Self::Error),
            101 => Ok(Self::OrderAccepted),
            102 => Ok(Self::OrderCanceled),
            103 => Ok(Self::OrderPartiallyFilled),
            104 => Ok(Self::OrderFilled),
            105 => Ok(Self::BookSnapshot),
            106 => Ok(Self::Trade),
            _ => Err(String::from("Invalid value")),
        }
    }
}

impl Message {
    pub fn decode(msg_type: u8, body: &mut BytesMut) -> Result<Self, ProtocolErrors> {
        match msg_type {
            1 => {
                return Ok(Message::CreateLimitOrder(
                    messages::CreateLimitOrder::decode(body)?,
                ));
            }
            2 => {
                return Ok(Message::CreateMarketOrder(
                    messages::CreateMarketOrder::decode(body)?,
                ));
            }
            3 => return Ok(Message::CancelOrder(messages::CancelOrder::decode(body)?)),
            4 => return Ok(Message::Error(errors::ProtocolErrors::decode(body)?)),

            101 => {
                return Ok(Message::OrderAccepted(
                    messages::engine_messages::OrderAccepted::decode(body)?,
                ));
            }

            102 => {
                return Ok(Message::OrderCanceled(
                    messages::engine_messages::OrderCanceled::decode(body)?,
                ));
            }

            103 => {
                return Ok(Message::OrderPartiallyFilled(
                    messages::engine_messages::OrderPartiallyFilled::decode(body)?,
                ));
            }

            104 => {
                return Ok(Message::OrderFilled(
                    messages::engine_messages::OrderFilled::decode(body)?,
                ));
            }

            105 => {
                return Ok(Message::BookSnapshot(
                    messages::engine_messages::BookSnapshot::decode(body)?,
                ));
            }

            106 => {
                return Ok(Message::Trade(
                    messages::engine_messages::Trade::decode(body)?,
                ));
            }

            _ => return Err(ProtocolErrors::InvalidMessageType),
        }
    }
}
