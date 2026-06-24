use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::records::scope::Scope;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

/// C++ `static void dumpBindings(NotNull<Scope> scope, ToStringOptions& opts)`.
pub fn dump_bindings(scope: *mut Scope, opts: &mut ToStringOptions) {
    let scope_ref = unsafe { &*scope };

    for (k, v) in &scope_ref.bindings {
        let d: String = to_string_type_id_to_string_options(v.type_id, opts);
        let key_str = unsafe { core::ffi::CStr::from_ptr(k.c_str()).to_string_lossy() };
        println!("\t{} : {}", key_str, d);
    }

    for child in &scope_ref.children {
        dump_bindings(*child, opts);
    }
}
