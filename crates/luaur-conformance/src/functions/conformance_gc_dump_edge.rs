use core::ffi::{c_char, c_void};

use crate::records::conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext;

pub unsafe extern "C" fn conformance_gc_dump_edge(
    context: *mut c_void,
    from: *mut c_void,
    to: *mut c_void,
    _name: *const c_char,
) {
    let context = &mut *(context as *mut ConformanceGcDumpEnumContext);
    context.edges.insert(from as usize, to as usize);
}
