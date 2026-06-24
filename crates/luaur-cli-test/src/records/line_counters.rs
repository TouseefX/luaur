#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[allow(non_camel_case_types)]
pub struct LineCounters {
    pub regularExecuted: u64,
    pub fallbackExecuted: u64,
    pub vmExitTaken: u64,
}

impl LineCounters {
    pub const fn new() -> Self {
        Self {
            regularExecuted: 0,
            fallbackExecuted: 0,
            vmExitTaken: 0,
        }
    }
}
