#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NumberedInstruction {
    pub inst_idx: u32,
    pub start_pos: u32,
    pub finish_pos: u32,
}

impl Default for NumberedInstruction {
    fn default() -> Self {
        Self {
            inst_idx: 0,
            start_pos: 0,
            finish_pos: 0,
        }
    }
}

#[allow(non_upper_case_globals)]
impl NumberedInstruction {
    pub const instIdx: u32 = 0;
    pub const startPos: u32 = 0;
    pub const finishPos: u32 = 0;
}
