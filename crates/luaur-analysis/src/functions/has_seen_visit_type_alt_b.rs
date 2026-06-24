use luaur_common::records::dense_hash_set::DenseHashSet;

#[allow(non_snake_case)]
pub fn has_seen_dense_hash_set_void_void(
    seen: &mut DenseHashSet<*mut core::ffi::c_void>,
    tv: *const core::ffi::c_void,
) -> bool {
    let ttv = tv as *mut core::ffi::c_void;

    if seen.contains(&ttv) {
        return true;
    }

    seen.insert(ttv);
    false
}
