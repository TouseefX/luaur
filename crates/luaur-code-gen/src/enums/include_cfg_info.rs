#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncludeCfgInfo {
    No,
    Yes,
}

impl Default for IncludeCfgInfo {
    fn default() -> Self {
        Self::No
    }
}

#[allow(non_upper_case_globals)]
impl IncludeCfgInfo {
    pub const No: Self = Self::No;
    pub const Yes: Self = Self::Yes;
}
