#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LineCounters {
    pub regularExecuted: u64,
    pub fallbackExecuted: u64,
    pub vmExitTaken: u64,
}
