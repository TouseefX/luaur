#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ConditionA64 {
    Equal,
    NotEqual,
    CarrySet,
    CarryClear,
    Minus,
    Plus,
    Overflow,
    NoOverflow,
    UnsignedGreater,
    UnsignedLessEqual,
    GreaterEqual,
    Less,
    Greater,
    LessEqual,
    Always,
    Count,
}

#[allow(non_upper_case_globals)]
impl ConditionA64 {
    pub const Equal: Self = Self::Equal;
    pub const NotEqual: Self = Self::NotEqual;
    pub const CarrySet: Self = Self::CarrySet;
    pub const CarryClear: Self = Self::CarryClear;
    pub const Minus: Self = Self::Minus;
    pub const Plus: Self = Self::Plus;
    pub const Overflow: Self = Self::Overflow;
    pub const NoOverflow: Self = Self::NoOverflow;
    pub const UnsignedGreater: Self = Self::UnsignedGreater;
    pub const UnsignedLessEqual: Self = Self::UnsignedLessEqual;
    pub const GreaterEqual: Self = Self::GreaterEqual;
    pub const Less: Self = Self::Less;
    pub const Greater: Self = Self::Greater;
    pub const LessEqual: Self = Self::LessEqual;
    pub const Always: Self = Self::Always;
    pub const Count: Self = Self::Count;

    pub const UnsignedLess: Self = Self::CarryClear;
    pub const UnsignedGreaterEqual: Self = Self::CarrySet;
}
