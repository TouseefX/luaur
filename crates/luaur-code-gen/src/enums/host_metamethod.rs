#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostMetamethod {
    Add,
    Sub,
    Mul,
    Div,
    Idiv,
    Mod,
    Pow,
    Minus,
    Equal,
    LessThan,
    LessEqual,
    Length,
    Concat,
}

#[allow(non_upper_case_globals)]
impl HostMetamethod {
    pub const Add: Self = Self::Add;
    pub const Sub: Self = Self::Sub;
    pub const Mul: Self = Self::Mul;
    pub const Div: Self = Self::Div;
    pub const Idiv: Self = Self::Idiv;
    pub const Mod: Self = Self::Mod;
    pub const Pow: Self = Self::Pow;
    pub const Minus: Self = Self::Minus;
    pub const Equal: Self = Self::Equal;
    pub const LessThan: Self = Self::LessThan;
    pub const LessEqual: Self = Self::LessEqual;
    pub const Length: Self = Self::Length;
    pub const Concat: Self = Self::Concat;
}
