use crate::records::vfs_navigator::VfsNavigator;
use alloc::string::String;

impl VfsNavigator {
    pub fn get_file_path(&self) -> String {
        self.real_path.clone()
    }
}
