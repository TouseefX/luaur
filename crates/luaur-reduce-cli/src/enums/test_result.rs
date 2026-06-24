#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum TestResult {
    BugFound, // We encountered the bug we are trying to isolate
    NoBug,    // We did not encounter the bug we are trying to isolate
}

impl TestResult {
    pub const BugFound: Self = Self::BugFound;
    pub const NoBug: Self = Self::NoBug;
}
