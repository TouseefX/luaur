#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConditionX64 {
    Overflow,
    NoOverflow,

    Carry,
    NoCarry,

    Below,
    BelowEqual,
    Above,
    AboveEqual,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    NotBelow,
    NotBelowEqual,
    NotAbove,
    NotAboveEqual,
    NotEqual,
    NotLess,
    NotLessEqual,
    NotGreater,
    NotGreaterEqual,

    Zero,
    NotZero,

    Parity,
    NotParity,

    Count,
}

#[allow(non_upper_case_globals)]
impl ConditionX64 {
    pub const Overflow: Self = Self::Overflow;
    pub const NoOverflow: Self = Self::NoOverflow;

    pub const Carry: Self = Self::Carry;
    pub const NoCarry: Self = Self::NoCarry;

    pub const Below: Self = Self::Below;
    pub const BelowEqual: Self = Self::BelowEqual;
    pub const Above: Self = Self::Above;
    pub const AboveEqual: Self = Self::AboveEqual;
    pub const Equal: Self = Self::Equal;
    pub const Less: Self = Self::Less;
    pub const LessEqual: Self = Self::LessEqual;
    pub const Greater: Self = Self::Greater;
    pub const GreaterEqual: Self = Self::GreaterEqual;

    pub const NotBelow: Self = Self::NotBelow;
    pub const NotBelowEqual: Self = Self::NotBelowEqual;
    pub const NotAbove: Self = Self::NotAbove;
    pub const NotAboveEqual: Self = Self::NotAboveEqual;
    pub const NotEqual: Self = Self::NotEqual;
    pub const NotLess: Self = Self::NotLess;
    pub const NotLessEqual: Self = Self::NotLessEqual;
    pub const NotGreater: Self = Self::NotGreater;
    pub const NotGreaterEqual: Self = Self::NotGreaterEqual;

    pub const Zero: Self = Self::Zero;
    pub const NotZero: Self = Self::NotZero;

    pub const Parity: Self = Self::Parity;
    pub const NotParity: Self = Self::NotParity;

    pub const Count: Self = Self::Count;
}
