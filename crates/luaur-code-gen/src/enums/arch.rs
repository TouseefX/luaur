#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X64,
    A64,
}

impl Default for Arch {
    fn default() -> Self {
        Self::X64
    }
}

#[allow(non_upper_case_globals)]
impl Arch {
    pub const X64: Self = Self::X64;
    pub const A64: Self = Self::A64;
}
