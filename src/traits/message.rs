pub trait Message {
    const MSG_TYPE: u8;
    const MSG_SIZE: usize;
}
