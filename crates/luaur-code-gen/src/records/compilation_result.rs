use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::records::proto_compilation_failure::ProtoCompilationFailure;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub result: CodeGenCompilationResult,
    pub proto_failures: Vec<ProtoCompilationFailure>,
}

impl Default for CompilationResult {
    fn default() -> Self {
        Self {
            result: CodeGenCompilationResult::Success,
            proto_failures: Vec::new(),
        }
    }
}
