use core::ffi::c_char;
use luaur_common::functions::edit_distance::editDistance;
use luaur_common::FInt::LuauSuggestionDistance;

pub fn fuzzy_match(str: &str, array: *const *const c_char, size: usize) -> *const c_char {
    let suggestion_distance = LuauSuggestionDistance.get() as usize;
    if suggestion_distance == 0 {
        return core::ptr::null();
    }

    let bytes = str.as_bytes();

    let mut best_distance = suggestion_distance;
    let mut best_match = size;

    for i in 0..size {
        let candidate = unsafe { *array.add(i) };
        if candidate.is_null() {
            continue;
        }

        let candidate_bytes = unsafe {
            let mut len = 0usize;
            while *candidate.add(len) != 0 {
                len += 1;
            }
            core::slice::from_raw_parts(candidate as *const u8, len)
        };

        let ed = editDistance(bytes, candidate_bytes);
        if ed <= best_distance {
            best_distance = ed;
            best_match = i;
        }
    }

    if best_match < size {
        unsafe { *array.add(best_match) }
    } else {
        core::ptr::null()
    }
}
