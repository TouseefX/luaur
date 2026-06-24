#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Host,
    A64,
    A64_NoFeatures,
    X64_Windows,
    X64_SystemV,
}

impl Default for Target {
    fn default() -> Self {
        Self::Host
    }
}

#[allow(non_upper_case_globals)]
impl Target {
    pub const Host: Self = Self::Host;
    pub const A64: Self = Self::A64;
    pub const A64_NoFeatures: Self = Self::A64_NoFeatures;
    pub const X64_Windows: Self = Self::X64_Windows;
    pub const X64_SystemV: Self = Self::X64_SystemV;
}
