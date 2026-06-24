#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CodeGenCompilationResult {
    Success = 0,
    NothingToCompile = 1,
    NotNativeModule = 2,
    CodeGenNotInitialized = 3,
    CodeGenOverflowInstructionLimit = 4,
    CodeGenOverflowBlockLimit = 5,
    CodeGenOverflowBlockInstructionLimit = 6,
    CodeGenAssemblerFinalizationFailure = 7,
    CodeGenLoweringFailure = 8,
    AllocationFailed = 9,
    Count = 10,
}

impl Default for CodeGenCompilationResult {
    fn default() -> Self {
        Self::Success
    }
}

#[allow(non_upper_case_globals)]
impl CodeGenCompilationResult {
    pub const Success: Self = Self::Success;
    pub const NothingToCompile: Self = Self::NothingToCompile;
    pub const NotNativeModule: Self = Self::NotNativeModule;
    pub const CodeGenNotInitialized: Self = Self::CodeGenNotInitialized;
    pub const CodeGenOverflowInstructionLimit: Self = Self::CodeGenOverflowInstructionLimit;
    pub const CodeGenOverflowBlockLimit: Self = Self::CodeGenOverflowBlockLimit;
    pub const CodeGenOverflowBlockInstructionLimit: Self =
        Self::CodeGenOverflowBlockInstructionLimit;
    pub const CodeGenAssemblerFinalizationFailure: Self = Self::CodeGenAssemblerFinalizationFailure;
    pub const CodeGenLoweringFailure: Self = Self::CodeGenLoweringFailure;
    pub const AllocationFailed: Self = Self::AllocationFailed;
    pub const Count: Self = Self::Count;
}
