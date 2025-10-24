use bytes::Bytes;
pub mod builder;
pub mod iterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout
    pub fn encode(&self) -> Bytes {
        unimplemented!()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(&self) -> Self {
        unimplemented!()
    }
}
