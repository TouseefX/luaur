#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum luarequire_ConfigStatus {
    CONFIG_ABSENT,
    CONFIG_AMBIGUOUS,
    CONFIG_PRESENT_JSON,
    CONFIG_PRESENT_LUAU,
}

#[allow(non_upper_case_globals)]
impl luarequire_ConfigStatus {
    pub const CONFIG_ABSENT: Self = Self::CONFIG_ABSENT;
    pub const CONFIG_AMBIGUOUS: Self = Self::CONFIG_AMBIGUOUS;
    pub const CONFIG_PRESENT_JSON: Self = Self::CONFIG_PRESENT_JSON;
    pub const CONFIG_PRESENT_LUAU: Self = Self::CONFIG_PRESENT_LUAU;
}

#[allow(non_camel_case_types)]
pub type luarequire_config_status = luarequire_ConfigStatus;
