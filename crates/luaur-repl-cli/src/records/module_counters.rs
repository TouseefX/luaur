use crate::records::function_counters::FunctionCounters;

#[derive(Debug, Clone)]
pub struct ModuleCounters {
    pub(crate) name: alloc::string::String,
    pub(crate) functions: alloc::vec::Vec<FunctionCounters>,
}

impl Default for ModuleCounters {
    fn default() -> Self {
        Self {
            name: alloc::string::String::new(),
            functions: alloc::vec::Vec::new(),
        }
    }
}
