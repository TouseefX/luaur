use crate::functions::from_utf_8::from_utf_8;

pub fn is_file(path: &str) -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileAttributesW, FILE_ATTRIBUTE_DIRECTORY, INVALID_FILE_ATTRIBUTES,
        };

        let path_u16 = from_utf_8(path);
        unsafe {
            let file_attributes = GetFileAttributesW(path_u16.as_ptr());
            if file_attributes == INVALID_FILE_ATTRIBUTES {
                return false;
            }
            (file_attributes & FILE_ATTRIBUTE_DIRECTORY) == 0
        }
    }

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::MetadataExt;
        std::fs::symlink_metadata(path)
            .map(|meta| (meta.mode() & 0xf000) == 0x8000)
            .unwrap_or(false)
    }
}
