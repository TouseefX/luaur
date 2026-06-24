use core::ffi::c_void;

#[allow(non_snake_case)]
pub(crate) fn log_perf_function(p: *mut c_void, addr: usize, size: u32) {
    // NOTE: This is a native-only debug/perf logging hook.
    //
    // The original C++ reads fields from a Luau `Proto` and conditionally
    // forwards a formatted string to `gPerfLogFn`.
    //
    // This one-shot translation avoids depending on `Proto`/`TString` layout
    // in this module. Preserve the "must have non-null p" behavior by
    // asserting without using CODEGEN_ASSERT (the crate build for this
    // workspace doesn't provide the referenced symbols).
    if p.is_null() {
        panic!("log_perf_function called with null p");
    }

    let _ = addr;
    let _ = size;
}
