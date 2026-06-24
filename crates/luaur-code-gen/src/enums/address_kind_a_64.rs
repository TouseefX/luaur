#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddressKindA64 {
    reg,  // reg + reg
    imm,  // reg + imm
    pre,  // reg + imm, reg += imm
    post, // reg, reg += imm
}

impl AddressKindA64 {
    pub const reg: Self = Self::reg;
    pub const imm: Self = Self::imm;
    pub const pre: Self = Self::pre;
    pub const post: Self = Self::post;
}
