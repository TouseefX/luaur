#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Kind {
    Imm26,
    Imm19,
    Imm14,
}

impl Kind {
    pub const Imm26: Kind = Kind::Imm26;
    pub const Imm19: Kind = Kind::Imm19;
    pub const Imm14: Kind = Kind::Imm14;
}
