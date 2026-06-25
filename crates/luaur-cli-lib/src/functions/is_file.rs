pub fn is_file(path: &str) -> bool {
    // Probe the filesystem through `std::fs` (like `is_directory`) rather than
    // raw Win32 wide-char FFI. The previous Windows path widened via
    // `from_utf_8` — whose UTF-16 buffer is NOT NUL-terminated — and handed it
    // to `GetFileAttributesW`, which reads until a NUL: undefined behaviour that
    // made existing files intermittently report as missing (e.g. require could
    // not descend into subdirectories). `symlink_metadata` + `is_file()` mirrors
    // the POSIX `lstat`/`S_IFREG` check and is correct on every platform.
    std::fs::symlink_metadata(path)
        .map(|meta| meta.is_file())
        .unwrap_or(false)
}
