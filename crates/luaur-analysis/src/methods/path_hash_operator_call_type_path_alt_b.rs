use crate::records::index::Index;
use crate::records::path_hash::PathHash;

impl PathHash {
    pub fn operator_call_3(&self, idx: &Index) -> usize {
        idx.index
    }
}
