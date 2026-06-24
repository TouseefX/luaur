use crate::type_aliases::config_status::ConfigStatus;
use luaur_cli_lib::enums::navigation_status::NavigationStatus;

pub fn convert_vfs_navigator_config_status(status: NavigationStatus) -> ConfigStatus {
    match status {
        NavigationStatus::Ambiguous => ConfigStatus::Ambiguous,
        NavigationStatus::NotFound => ConfigStatus::Absent,
        NavigationStatus::Success => ConfigStatus::Absent,
    }
}
