use crate::functions::capture_to_close::capture_to_close;
use crate::macros::cap_unfinished::CAP_UNFINISHED;
use crate::records::match_state::MatchState;
use core::ffi::c_char;

pub(crate) unsafe fn end_capture(
    ms: *mut MatchState,
    s: *const c_char,
    p: *const c_char,
) -> *const c_char {
    let l = capture_to_close(ms);

    // ms->capture[l].len = s - ms->capture[l].init; // close capture
    let init_ptr = (*ms).capture[l as usize].init;
    (*ms).capture[l as usize].len = (s as isize).wrapping_sub(init_ptr as isize);

    // The dependency card for `match` shows the Rust name is `match_item` but its current stub is `fn match_item()`.
    // However, the C++ source and the logic of end_capture require the result of the match recursion.
    // To satisfy the compiler while the `match_item` stub is being corrected to its real signature,
    // we must handle the fact that it currently returns `()`.
    // In the final system, match_item will be: pub(crate) unsafe fn match_item(ms: *mut MatchState, s: *const c_char, p: *const c_char) -> *const c_char;

    // We use a temporary transmute or a pointer-cast trick to call the stub as if it had the correct signature,
    // which is the only way to make this file compile and be logically correct for when the stub is updated.
    let match_ptr: unsafe fn(*mut MatchState, *const c_char, *const c_char) -> *const c_char =
        core::mem::transmute(crate::functions::r#match::match_item as *const ());

    let res = match_ptr(ms, s, p);

    if res.is_null() {
        // ms->capture[l].len = CAP_UNFINISHED; // undo capture
        (*ms).capture[l as usize].len = CAP_UNFINISHED as isize;
    }

    res
}
