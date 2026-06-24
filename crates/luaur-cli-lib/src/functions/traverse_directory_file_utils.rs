pub fn traverse_directory_mut(path: &str, callback: &dyn Fn(&str)) -> bool {
    // Faithful to FileUtils.cpp `traverseDirectory`: under `_WIN32` the path is
    // widened to UTF-16 and walked via FindFirstFileW; on POSIX the recursive
    // helper takes the byte path directly.
    #[cfg(windows)]
    {
        use crate::functions::from_utf_8::from_utf_8;
        use crate::functions::traverse_directory_rec_file_utils::traverse_directory_rec_wstring_function_void_const_string_name;

        let path_utf16 = from_utf_8(path);
        traverse_directory_rec_wstring_function_void_const_string_name(&path_utf16, callback)
    }

    #[cfg(not(windows))]
    {
        use crate::functions::traverse_directory_rec_file_utils_alt_b::traverse_directory_rec_string_function_void_const_string_name;

        traverse_directory_rec_string_function_void_const_string_name(path, callback)
    }
}
