#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionWin {
    pub begin_offset: u32,
    pub end_offset: u32,
    pub unwind_info_offset: u32,
}

#[allow(non_upper_case_globals)]
impl UnwindFunctionWin {
    pub const beginOffset: u32 = 0;
    pub const endOffset: u32 = 0;
    pub const unwindInfoOffset: u32 = 0;
}
