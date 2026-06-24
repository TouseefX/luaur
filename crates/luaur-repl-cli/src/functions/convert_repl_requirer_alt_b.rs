use luaur_cli_lib::enums::config_status::ConfigStatus;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum luarequire_ConfigStatus {
    CONFIG_AMBIGUOUS = 0,
    CONFIG_PRESENT_JSON = 1,
    CONFIG_PRESENT_LUAU = 2,
    CONFIG_ABSENT = 3,
}

pub fn convert(status: ConfigStatus) -> luarequire_ConfigStatus {
    if status == ConfigStatus::Ambiguous {
        luarequire_ConfigStatus::CONFIG_AMBIGUOUS
    } else if status == ConfigStatus::PresentJson {
        luarequire_ConfigStatus::CONFIG_PRESENT_JSON
    } else if status == ConfigStatus::PresentLuau {
        luarequire_ConfigStatus::CONFIG_PRESENT_LUAU
    } else {
        luarequire_ConfigStatus::CONFIG_ABSENT
    }
}
