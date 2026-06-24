use crate::enums::config_status::ConfigStatus;
use crate::functions::read_file::read_file;
use crate::records::vfs_navigator::VfsNavigator;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl VfsNavigator {
    pub fn get_config(&self) -> Option<String> {
        let status = self.get_config_status();
        LUAU_ASSERT!(status == ConfigStatus::PresentJson || status == ConfigStatus::PresentLuau);

        if status == ConfigStatus::PresentJson {
            // Luau::kConfigName
            read_file(&self.get_config_path(".luaurc"))
        } else if status == ConfigStatus::PresentLuau {
            // Luau::kLuauConfigName
            read_file(&self.get_config_path(".config.luau"))
        } else {
            LUAU_UNREACHABLE!();
            None
        }
    }
}
