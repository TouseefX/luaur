use std::ffi::c_void;
use std::sync::atomic::Ordering;

use crate::functions::conformance_reference_dtor_hits::CONFORMANCE_REFERENCE_DTOR_HITS;

pub unsafe extern "C" fn conformance_reference_dtor(_data: *mut c_void) {
    CONFORMANCE_REFERENCE_DTOR_HITS.fetch_add(1, Ordering::SeqCst);
}
