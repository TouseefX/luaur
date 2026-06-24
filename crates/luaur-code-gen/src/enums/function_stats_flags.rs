#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FunctionStatsFlags {
    FunctionStats_Enable = 1 << 0,
    FunctionStats_BytecodeSummary = 1 << 1,
}

#[allow(non_upper_case_globals)]
impl FunctionStatsFlags {
    pub const FunctionStats_Enable: Self = Self::FunctionStats_Enable;
    pub const FunctionStats_BytecodeSummary: Self = Self::FunctionStats_BytecodeSummary;
}

impl core::ops::BitOr for FunctionStatsFlags {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as i32) | (rhs as i32)
    }
}

impl core::ops::BitOr<FunctionStatsFlags> for i32 {
    type Output = i32;

    fn bitor(self, rhs: FunctionStatsFlags) -> Self::Output {
        self | (rhs as i32)
    }
}
