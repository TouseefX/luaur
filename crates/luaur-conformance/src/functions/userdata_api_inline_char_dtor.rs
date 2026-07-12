use core::ffi::c_void;
use core::sync::atomic::Ordering;

use crate::functions::userdata_api_dtor_hits::USERDATA_API_DTOR_HITS;

pub unsafe extern "C" fn userdata_api_inline_char_dtor(data: *mut c_void) {
    USERDATA_API_DTOR_HITS.fetch_add(
        *(data as *const core::ffi::c_char) as u8 as i32,
        Ordering::SeqCst,
    );
}
