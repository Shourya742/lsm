use std::path::Path;

use crate::{
    block::builder::BlockBuilder,
    key::KeySlice,
    lsm_storage::BlockCache,
    table::{BlockMeta, SsTable},
};

/// Builds an SSTable from key-value pairs.
pub struct SsTableBuilder {
    builder: BlockBuilder,
    first_key: Vec<u8>,
    last_key: Vec<u8>,
    data: Vec<u8>,
    pub(crate) meta: Vec<BlockMeta>,
    block_size: usize,
}

impl SsTableBuilder {
    /// Create a builder based on target block size.
    pub fn new(block_size: usize) -> Self {
        unimplemented!()
    }

    /// Adds a key-value pair to SSTable
    ///
    /// Note: You should split a new block when the current block is full. (`std::mem::replace`) may
    /// be helpful here.
    pub fn add(&mut self, key: KeySlice, value: &[u8]) {
        unimplemented!()
    }

    /// Get the estimated size of the SSTable.
    ///
    /// Since the data blocks contains much more data than meta blocks. Just return the size of data.
    pub fn estimate_size(&self) -> usize {
        unimplemented!()
    }

    /// Builds the SSTable and writes it to the given path. Use the `FileObject` structure to manipulate the disk objects.
    pub fn build(
        mut self,
        id: usize,
        block_cache: Option<Arc<BlockCache>>,
        path: impl AsRef<Path>,
    ) -> Result<SsTable> {
        unimplemented!()
    }

    pub(crate) fn build_for_test(self, path: impl AsRef<Path>) -> Result<SsTable> {
        self.build(0, None, path)
    }
}
