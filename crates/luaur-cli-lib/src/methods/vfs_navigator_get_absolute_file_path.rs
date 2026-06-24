use crate::records::vfs_navigator::VfsNavigator;
use alloc::string::String;

impl VfsNavigator {
    pub fn get_absolute_file_path(&self) -> String {
        self.absolute_real_path.clone()
    }
}
