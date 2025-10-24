use crate::{
    block::iterator::BlockIterator, iterators::StorageIterator, key::KeySlice, table::SsTable,
};

/// An iterator over the contents of an SSTable.
pub struct SsTableIterator {
    table: Arc<SsTable>,
    blk_iter: BlockIterator,
    blk_idx: usize,
}

impl SsTableIterator {
    /// Create a new iterator and seek to the first key-value pair in the first data block.
    pub fn create_and_seek_to_first(table: Arc<SsTable>) -> Result<Self> {
        unimplemented!()
    }

    /// Seek to the first key-value pair in the first data block.
    pub fn seek_to_first(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// Create a new iterator and seek to the first key-value pair which == `key`
    pub fn create_and_seek_to_key(table: Arc<SsTable>, key: KeySlice) -> Result<Self> {
        unimplemented!()
    }

    /// Seek to the first key-value pair which >= `key`
    pub fn seek_to_key(&mut self, key: KeySlice) -> Result<()> {
        unimplemented!()
    }
}

impl StorageIterator for SsTable {
    type KeyType<'a> = KeySlice<'a>;

    /// Return the `key` that's held by the underlying block iterator.
    fn key(&self) -> Self::KeyType<'_> {
        unimplemented!()
    }

    /// Return the `value` that's held by the underlying block iterator.
    fn value(&self) -> &[u8] {
        unimplemented!()
    }

    /// Return whether the current block iterator is valid or not.
    fn is_valid(&self) -> bool {
        unimplemented!()
    }

    /// Move to the next `key` in the block.
    fn next(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
