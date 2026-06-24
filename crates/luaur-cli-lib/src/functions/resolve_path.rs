pub fn resolve_path(path: &str, base_file_path: &str) -> Option<alloc::string::String> {
    let base_file_path_parent = crate::functions::get_parent_path::get_parent_path(base_file_path)?;
    let joined = crate::functions::join_paths_file_utils_alt_b::join_paths_string_view_string_view(
        &base_file_path_parent,
        path,
    );
    Some(crate::functions::normalize_path::normalize_path(&joined))
}
