#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct StackItem {
    pub block_idx: u32,
    pub it_pos: u32,
}

#[allow(non_upper_case_globals)]
impl StackItem {
    pub const blockIdx: u32 = 0;
    pub const itPos: u32 = 0;
}
