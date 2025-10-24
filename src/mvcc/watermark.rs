use std::collections::BTreeMap;

pub struct Watermark {
    readers: BTreeMap<u64, usize>,
}

impl Default for Watermark {
    fn default() -> Self {
        Self::new()
    }
}

impl Watermark {
    pub fn new() -> Self {
        Self {
            readers: BTreeMap::new(),
        }
    }

    pub fn add_reader(&mut self, ts: u64) {}

    pub fn remove_reader(&mut self, ts: u64) {}

    pub fn num_retained_snapshots(&self) -> usize {
        self.readers.len()
    }

    pub fn watermark(&self) -> Option<u64> {
        Some(0)
    }
}
