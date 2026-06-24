#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CodeGenCounter {
    RegularBlockExecuted = 1,
    FallbackBlockExecuted = 2,
    VmExitTaken = 3,
}

#[allow(non_upper_case_globals)]
impl CodeGenCounter {
    pub const RegularBlockExecuted: Self = Self::RegularBlockExecuted;
    pub const FallbackBlockExecuted: Self = Self::FallbackBlockExecuted;
    pub const VmExitTaken: Self = Self::VmExitTaken;
}
