#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CompilationStats {
    pub bytecode_size_bytes: usize,
    pub native_code_size_bytes: usize,
    pub native_data_size_bytes: usize,
    pub native_metadata_size_bytes: usize,
    pub functions_total: u32,
    pub functions_compiled: u32,
    pub functions_bound: u32,
}

#[allow(non_upper_case_globals)]
impl CompilationStats {
    pub const bytecodeSizeBytes: usize = 0;
    pub const nativeCodeSizeBytes: usize = 0;
    pub const nativeDataSizeBytes: usize = 0;
    pub const nativeMetadataSizeBytes: usize = 0;
    pub const functionsTotal: u32 = 0;
    pub const functionsCompiled: u32 = 0;
    pub const functionsBound: u32 = 0;
}
