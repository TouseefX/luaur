#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CodeGenFlags {
    CodeGen_OnlyNativeModules = 1 << 0,
    CodeGen_ColdFunctions = 1 << 1,
}

impl CodeGenFlags {
    pub const CodeGen_OnlyNativeModules: CodeGenFlags = CodeGenFlags::CodeGen_OnlyNativeModules;
    pub const CodeGen_ColdFunctions: CodeGenFlags = CodeGenFlags::CodeGen_ColdFunctions;
}
