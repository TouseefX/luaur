use luaur_cli_lib::enums::config_status::ConfigStatus;
pub use luaur_require::enums::luarequire_config_status::luarequire_ConfigStatus;

pub fn convert_vfs_navigator_config_status(status: ConfigStatus) -> luarequire_ConfigStatus {
    match status {
        ConfigStatus::Ambiguous => luarequire_ConfigStatus::CONFIG_AMBIGUOUS,
        ConfigStatus::PresentJson => luarequire_ConfigStatus::CONFIG_PRESENT_JSON,
        ConfigStatus::PresentLuau => luarequire_ConfigStatus::CONFIG_PRESENT_LUAU,
        ConfigStatus::Absent => luarequire_ConfigStatus::CONFIG_ABSENT,
    }
}
