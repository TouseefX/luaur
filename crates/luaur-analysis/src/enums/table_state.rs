#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum TableState {
    Sealed,
    Unsealed,
    Free,
    Generic,
}

impl TableState {
    pub const Sealed: Self = Self::Sealed;
    pub const Unsealed: Self = Self::Unsealed;
    pub const Free: Self = Self::Free;
    pub const Generic: Self = Self::Generic;
}
