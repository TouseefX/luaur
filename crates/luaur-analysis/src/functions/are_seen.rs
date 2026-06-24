use std::collections::BTreeSet;

#[allow(non_snake_case)]
pub fn are_seen(
    seen: &mut BTreeSet<(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
    lhs: *const core::ffi::c_void,
    rhs: *const core::ffi::c_void,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let p = (lhs as *mut core::ffi::c_void, rhs as *mut core::ffi::c_void);
    if seen.contains(&p) {
        return true;
    }

    seen.insert(p);
    false
}
