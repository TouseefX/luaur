extern crate alloc;

use crate::records::function_counters::FunctionCounters;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct ModuleCounters {
    pub(crate) name: String,
    pub(crate) functions: Vec<FunctionCounters>,
}
