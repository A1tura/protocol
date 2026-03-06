use bytes::BytesMut;

pub trait Decode {
    fn decode(buf: &mut BytesMut) -> Self;
}
