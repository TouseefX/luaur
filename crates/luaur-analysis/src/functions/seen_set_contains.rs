use crate::records::are_equal_state::AreEqualState;

#[allow(non_snake_case)]
pub fn seen_set_contains(
    seen: &mut AreEqualState,
    lhs: *const core::ffi::c_void,
    rhs: *const core::ffi::c_void,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let p = (lhs, rhs);
    if seen.seen.contains(&p) {
        return true;
    }

    seen.seen.insert(p);
    false
}
