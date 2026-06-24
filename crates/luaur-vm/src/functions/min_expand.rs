use crate::functions::r#match::match_item as match_fn;
use crate::functions::singlematch::singlematch;
use crate::records::match_state::MatchState;
use core::ffi::c_char;

pub(crate) unsafe fn min_expand(
    ms: *mut MatchState,
    mut s: *const c_char,
    p: *const c_char,
    ep: *const c_char,
) -> *const c_char {
    // The dependencies match_item and singlematch are currently stubs returning ().
    // We must transmute them to their real signatures to allow this logic to compile and function
    // correctly once the stubs are updated, as per the established pattern in this crate.
    let match_ptr: unsafe fn(*mut MatchState, *const c_char, *const c_char) -> *const c_char =
        core::mem::transmute(match_fn as *const ());

    let singlematch_ptr: unsafe fn(
        *mut MatchState,
        *const c_char,
        *const c_char,
        *const c_char,
    ) -> bool = core::mem::transmute(singlematch as *const ());

    loop {
        let res = match_ptr(ms, s, ep.add(1));

        if !res.is_null() {
            return res;
        } else if singlematch_ptr(ms, s, p, ep) {
            // try with one more repetition
            s = s.add(1);
        } else {
            return core::ptr::null();
        }
    }
}
