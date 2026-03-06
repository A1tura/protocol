use bytes::BytesMut;


pub trait Encode {
    fn encode(&self, buf: &mut BytesMut);
}
