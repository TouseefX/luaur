#[cfg(windows)]
pub fn is_directory(path: &str) -> bool {
    use std::os::windows::fs::MetadataExt;
    std::fs::symlink_metadata(path)
        .map(|meta| (meta.file_attributes() & 0x10) != 0)
        .unwrap_or(false)
}

#[cfg(not(windows))]
pub fn is_directory(path: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    std::fs::symlink_metadata(path)
        .map(|meta| (meta.mode() & 0xf000) == 0x4000)
        .unwrap_or(false)
}
