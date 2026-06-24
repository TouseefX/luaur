#[derive(Debug, Clone)]
pub struct BytecodeCompilerFixture {
    pub(crate) strings: alloc::vec::Vec<alloc::string::String>,
}

impl BytecodeCompilerFixture {
    pub fn new() -> Self {
        Self {
            strings: alloc::vec::Vec::new(),
        }
    }
}
