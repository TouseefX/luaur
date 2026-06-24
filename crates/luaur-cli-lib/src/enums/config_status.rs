#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum ConfigStatus {
    Absent,
    Ambiguous,
    PresentJson,
    PresentLuau,
}

impl ConfigStatus {
    pub const Absent: Self = Self::Absent;
    pub const Ambiguous: Self = Self::Ambiguous;
    pub const PresentJson: Self = Self::PresentJson;
    pub const PresentLuau: Self = Self::PresentLuau;
}
