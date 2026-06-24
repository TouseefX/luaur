use crate::functions::r#match::match_item;
use crate::functions::singlematch::singlematch;
use crate::records::match_state::MatchState;
use core::ffi::c_char;

pub(crate) unsafe fn max_expand(
    ms: *mut MatchState,
    s: *const c_char,
    p: *const c_char,
    ep: *const c_char,
) -> *const c_char {
    let mut i: isize = 0; // counts maximum expand for item

    // The dependencies singlematch and match_item are currently stubs returning ().
    // We must transmute them to their real signatures to allow this logic to compile
    // and function correctly once the stubs are updated.
    let singlematch_ptr: unsafe fn(
        *mut MatchState,
        *const c_char,
        *const c_char,
        *const c_char,
    ) -> i32 = core::mem::transmute(singlematch as *const ());

    let match_ptr: unsafe fn(*mut MatchState, *const c_char, *const c_char) -> *const c_char =
        core::mem::transmute(match_item as *const ());

    // while (singlematch(ms, s + i, p, ep))
    //     i++;
    while singlematch_ptr(ms, s.offset(i), p, ep) != 0 {
        i += 1;
    }

    // keeps trying to match with the maximum repetitions
    while i >= 0 {
        let res = match_ptr(ms, s.offset(i), ep.offset(1));

        if !res.is_null() {
            return res;
        }

        i -= 1; // else didn't match; reduce 1 repetition to try again
    }

    core::ptr::null()
}
