#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct BytecodeRes {
    pub(crate) inlinee_bytecode: alloc::string::String,
    pub(crate) caller_bytecode: alloc::string::String,
    pub(crate) string_table: alloc::vec::Vec<alloc::string::String>,
}
