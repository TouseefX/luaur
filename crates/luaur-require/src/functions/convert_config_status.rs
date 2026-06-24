use crate::enums::config_status::ConfigStatus;

#[allow(non_camel_case_types)]
pub type luarequire_ConfigStatus = i32;

pub const CONFIG_ABSENT: luarequire_ConfigStatus = 0;
pub const CONFIG_AMBIGUOUS: luarequire_ConfigStatus = 1;
pub const CONFIG_PRESENT_JSON: luarequire_ConfigStatus = 2;
pub const CONFIG_PRESENT_LUAU: luarequire_ConfigStatus = 3;

pub(crate) fn convert_config_status(status: luarequire_ConfigStatus) -> ConfigStatus {
    if status == CONFIG_PRESENT_JSON {
        return ConfigStatus::PresentJson;
    }
    if status == CONFIG_PRESENT_LUAU {
        return ConfigStatus::PresentLuau;
    }
    if status == CONFIG_AMBIGUOUS {
        return ConfigStatus::Ambiguous;
    }

    ConfigStatus::Absent
}
