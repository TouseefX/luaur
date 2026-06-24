use crate::functions::get_extension::get_extension;
use crate::functions::is_directory::is_directory;
use crate::functions::normalize_path::normalize_path;
use crate::functions::traverse_directory_file_utils::traverse_directory_mut;
use core::ffi::CStr;

pub fn get_source_files(
    argc: i32,
    argv: *mut *mut core::ffi::c_char,
) -> alloc::vec::Vec<alloc::string::String> {
    let mut files = alloc::vec::Vec::new();

    for i in 1..argc as usize {
        let arg_ptr = unsafe { *argv.add(i) };
        if arg_ptr.is_null() {
            continue;
        }

        let arg = unsafe { CStr::from_ptr(arg_ptr).to_string_lossy() };

        if arg == "--program-args" || arg == "-a" {
            return files;
        }

        // Treat '-' as a special file whose source is read from stdin
        // All other arguments that start with '-' are skipped
        if arg.starts_with('-') && arg.len() > 1 {
            continue;
        }

        let normalized = normalize_path(&arg);

        if is_directory(&normalized) {
            // Use a Cell or RefCell to allow mutation of the captured vector within the Fn closure,
            // or use a raw pointer if we are sure about the safety context of traverse_directory_mut.
            // Since traverse_directory_mut takes &dyn Fn, we use a RefCell for interior mutability.
            let files_ref = std::cell::RefCell::new(&mut files);

            traverse_directory_mut(&normalized, &|name: &str| {
                let ext = get_extension(name);
                if ext == ".lua" || ext == ".luau" {
                    files_ref
                        .borrow_mut()
                        .push(alloc::string::String::from(name));
                }
            });
        } else {
            files.push(normalized);
        }
    }

    files
}
