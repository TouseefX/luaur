use crate::macros::get_cwd::getCwd;

pub fn find_conformance_source_dir() -> String {
    // Scripts vendored alongside the crate (standalone / published-repo layout).
    // Checked first and cwd-independent; falls through to the cwd-walk below when
    // building inside the original workspace (where luau/tests/conformance exists).
    let vendored = concat!(env!("CARGO_MANIFEST_DIR"), "/conformance");
    if std::path::Path::new(vendored).is_dir() {
        return vendored.to_owned();
    }

    let mut buf = [0i8; 4096];
    unsafe {
        if getCwd(buf.as_mut_ptr(), buf.len() as core::ffi::c_int).is_null() {
            return String::new();
        }
    }

    let cwd = unsafe {
        core::ffi::CStr::from_ptr(buf.as_ptr())
            .to_string_lossy()
            .into_owned()
    };

    let mut dir = std::path::PathBuf::from(&cwd);

    for _ in 0..20 {
        let local_conformance = dir.join("luau/tests/conformance");
        if let Ok(meta) = std::fs::metadata(&local_conformance) {
            if meta.is_dir() {
                return local_conformance.to_string_lossy().into_owned();
            }
        }

        if let Ok(meta) = std::fs::metadata(dir.join("Client/content")) {
            if meta.is_dir() {
                return dir
                    .join("Client/Luau/tests/conformance")
                    .to_string_lossy()
                    .into_owned();
            }
        }

        if !dir.pop() {
            break;
        }
    }

    String::new()
}
