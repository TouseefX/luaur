#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrCondition {
    Equal,
    NotEqual,
    Less,
    NotLess,
    LessEqual,
    NotLessEqual,
    Greater,
    NotGreater,
    GreaterEqual,
    NotGreaterEqual,

    UnsignedLess,
    UnsignedLessEqual,
    UnsignedGreater,
    UnsignedGreaterEqual,

    Count,
}

#[allow(non_upper_case_globals)]
impl IrCondition {
    pub const Equal: Self = Self::Equal;
    pub const NotEqual: Self = Self::NotEqual;
    pub const Less: Self = Self::Less;
    pub const NotLess: Self = Self::NotLess;
    pub const LessEqual: Self = Self::LessEqual;
    pub const NotLessEqual: Self = Self::NotLessEqual;
    pub const Greater: Self = Self::Greater;
    pub const NotGreater: Self = Self::NotGreater;
    pub const GreaterEqual: Self = Self::GreaterEqual;
    pub const NotGreaterEqual: Self = Self::NotGreaterEqual;

    pub const UnsignedLess: Self = Self::UnsignedLess;
    pub const UnsignedLessEqual: Self = Self::UnsignedLessEqual;
    pub const UnsignedGreater: Self = Self::UnsignedGreater;
    pub const UnsignedGreaterEqual: Self = Self::UnsignedGreaterEqual;

    pub const Count: Self = Self::Count;
}
