use crate::functions::has_suffix::has_suffix;
use crate::functions::is_absolute_path::is_absolute_path;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

const K_INIT_SUFFIXES: &[&str] = &["/init.luau", "/init.lua"];
const K_SUFFIXES: &[&str] = &[".luau", ".lua"];

pub fn get_module_path(file_path: &str) -> alloc::string::String {
    let file_path_normalized = file_path.replace('\\', "/");

    let mut path_view: &str = &file_path_normalized;

    if is_absolute_path(path_view) {
        let first_slash = path_view.find('/');
        LUAU_ASSERT!(first_slash.is_some());
        if let Some(idx) = first_slash {
            path_view = &path_view[idx..];
        }
    }

    for &suffix in K_INIT_SUFFIXES {
        if has_suffix(path_view, suffix) {
            path_view = &path_view[..path_view.len() - suffix.len()];
            return alloc::string::String::from(path_view);
        }
    }

    for &suffix in K_SUFFIXES {
        if has_suffix(path_view, suffix) {
            path_view = &path_view[..path_view.len() - suffix.len()];
            return alloc::string::String::from(path_view);
        }
    }

    alloc::string::String::from(path_view)
}
