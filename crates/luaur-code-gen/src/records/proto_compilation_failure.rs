extern crate alloc;

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtoCompilationFailure {
    pub result: CodeGenCompilationResult,
    pub debugname: String,
    pub line: i32,
}

impl Default for ProtoCompilationFailure {
    fn default() -> Self {
        Self {
            result: CodeGenCompilationResult::Success,
            debugname: String::new(),
            line: -1,
        }
    }
}
