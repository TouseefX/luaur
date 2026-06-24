#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct BytecodeInlinerFixture {
    pub(crate) strings: Vec<alloc::string::String>,
}

impl BytecodeInlinerFixture {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BytecodeRes {
    pub(crate) inlinee_bytecode: alloc::string::String,
    pub(crate) caller_bytecode: alloc::string::String,
    pub(crate) string_table: Vec<alloc::string::String>,
}
