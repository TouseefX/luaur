#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncludeRegFlowInfo {
    No,
    Yes,
}

impl Default for IncludeRegFlowInfo {
    fn default() -> Self {
        Self::No
    }
}

#[allow(non_upper_case_globals)]
impl IncludeRegFlowInfo {
    pub const No: Self = Self::No;
    pub const Yes: Self = Self::Yes;
}
