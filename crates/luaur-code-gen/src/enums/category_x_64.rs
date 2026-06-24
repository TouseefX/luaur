#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CategoryX64 {
    reg,
    mem,
    imm,
}

#[allow(non_upper_case_globals)]
impl CategoryX64 {
    pub const reg: Self = Self::reg;
    pub const mem: Self = Self::mem;
    pub const imm: Self = Self::imm;
}
