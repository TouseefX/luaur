use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct AliasCycleTracker {
    pub(crate) seen: DenseHashSet<String>,
    pub(crate) ordered: Vec<String>,
}
