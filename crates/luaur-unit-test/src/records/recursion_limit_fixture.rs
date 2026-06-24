use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;

#[derive(Debug, Clone)]
pub struct RecursionLimitFixture {
    pub(crate) bcb: BytecodeBuilder,
    pub(crate) reps: i32,
    pub(crate) find_limit: bool,
}
