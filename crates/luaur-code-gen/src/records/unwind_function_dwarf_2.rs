#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UnwindFunctionDwarf2 {
    pub begin_offset: u32,
    pub end_offset: u32,
    pub fde_entry_start_pos: u32,
}

#[allow(non_upper_case_globals)]
impl UnwindFunctionDwarf2 {
    pub const beginOffset: u32 = 0;
    pub const endOffset: u32 = 0;
    pub const fdeEntryStartPos: u32 = 0;
}
