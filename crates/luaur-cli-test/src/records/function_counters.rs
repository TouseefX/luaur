extern crate alloc;

use crate::records::line_counters::LineCounters;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct FunctionCounters {
    pub(crate) name: String,
    pub(crate) counters: luaur_common::records::dense_hash_map::DenseHashMap<i32, LineCounters>,
}
