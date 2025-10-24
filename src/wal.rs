use std::{fs::File, path::Path, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;

use crate::key::KeySlice;

pub struct Wal {
    file: Arc<Mutex<File>>,
}

impl Wal {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        unimplemented!()
    }

    pub fn recover(path: impl AsRef<Path>, skiplist: &SkipMap<Bytes, Bytes>) -> Result<Self> {
        unimplemented!()
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    pub fn put_batch(&self, data: &[(KeySlice, &[u8])]) -> Result<()> {
        unimplemented!()
    }

    pub fn sync(&self) -> Result<()> {
        unimplemented!()
    }
}
