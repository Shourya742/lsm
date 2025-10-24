use crate::key::{KeySlice, KeyVec};

/// Builds a block.
pub struct BlockBuilder {
    /// Offsets of each key-value entries.
    offset: Vec<u16>,
    /// All serialized key-value pairs in the block.
    data: Vec<u8>,
    /// The expected block size.
    block_size: usize,
    /// The first key in the block.
    first_key: KeyVec,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        unimplemented!()
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    /// You may find the `bytes::BufMut` trait useful for manipulating binary data.
    #[must_use]
    pub fn add(&mut self, key: KeySlice, value: &[u8]) -> bool {
        unimplemented!()
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        unimplemented!()
    }
}
