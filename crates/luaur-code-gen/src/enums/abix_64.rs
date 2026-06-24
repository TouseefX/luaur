#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ABIX64 {
    Windows,
    SystemV,
}

impl Default for ABIX64 {
    fn default() -> Self {
        Self::Windows
    }
}

#[allow(non_upper_case_globals)]
impl ABIX64 {
    pub const Windows: ABIX64 = ABIX64::Windows;
    pub const SystemV: ABIX64 = ABIX64::SystemV;
}
