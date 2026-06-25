pub fn traverse_directory_mut(path: &str, callback: &dyn Fn(&str)) -> bool {
    // Faithful to FileUtils.cpp `traverseDirectory`. Both platforms route to the
    // byte-path recursive helper: its POSIX branch walks via `std::fs::read_dir`
    // and its Windows branch via `FindFirstFileW` (correctly NUL-terminating the
    // search path). The previous Windows route went to a wstring helper that was
    // only a stub (returned `false`, never enumerated), so directory traversal
    // silently did nothing on Windows.
    use crate::functions::traverse_directory_rec_file_utils_alt_b::traverse_directory_rec_string_function_void_const_string_name;

    traverse_directory_rec_string_function_void_const_string_name(path, callback)
}
