#[allow(non_snake_case)]
pub fn unsee(
    seen: &mut std::collections::HashSet<*mut core::ffi::c_void>,
    tv: *const core::ffi::c_void,
) {
    let ttv = tv as *mut core::ffi::c_void;
    seen.remove(&ttv);
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use unsee as unsee_unordered_set_void_void;
