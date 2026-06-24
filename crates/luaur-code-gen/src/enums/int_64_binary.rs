#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Int64Binary {
    Add,
    Sub,
    Mul,
    Div,
    Idiv,
    Udiv,
    Rem,
    Urem,
    Mod,
}

#[allow(non_upper_case_globals)]
impl Int64Binary {
    pub const Add: Self = Self::Add;
    pub const Sub: Self = Self::Sub;
    pub const Mul: Self = Self::Mul;
    pub const Div: Self = Self::Div;
    pub const Idiv: Self = Self::Idiv;
    pub const Udiv: Self = Self::Udiv;
    pub const Rem: Self = Self::Rem;
    pub const Urem: Self = Self::Urem;
    pub const Mod: Self = Self::Mod;
}
