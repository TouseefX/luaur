#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeMapping {
    pub ir_location: u32,
    pub asm_location: u32,
}

#[allow(non_upper_case_globals)]
impl BytecodeMapping {
    pub const irLocation: u32 = 0;
    pub const asmLocation: u32 = 0;
}
