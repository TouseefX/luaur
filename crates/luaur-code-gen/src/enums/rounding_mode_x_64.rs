#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoundingModeX64 {
    RoundToNearestEven = 0b00,
    RoundToNegativeInfinity = 0b01,
    RoundToPositiveInfinity = 0b10,
    RoundToZero = 0b11,
}

#[allow(non_upper_case_globals)]
impl RoundingModeX64 {
    pub const RoundToNearestEven: Self = Self::RoundToNearestEven;
    pub const RoundToNegativeInfinity: Self = Self::RoundToNegativeInfinity;
    pub const RoundToPositiveInfinity: Self = Self::RoundToPositiveInfinity;
    pub const RoundToZero: Self = Self::RoundToZero;
}
