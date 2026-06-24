//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1910:dump`
//! Source: `Analysis/src/ToString.cpp:1910-1923` (hand-ported)

use crate::functions::dump_options::dump_options;
use crate::type_aliases::scope_ptr_scope::ScopePtr;
use alloc::string::{String, ToString};
use core::ffi::c_char;

/// C++ `std::string dump(const ScopePtr& scope, const char* name)`.
pub fn dump_scope_ptr_c_char(scope: &ScopePtr, name: *const c_char) -> String {
    unsafe {
        let name_str = core::ffi::CStr::from_ptr(name)
            .to_string_lossy()
            .to_string();
        let binding = scope.linear_search_for_binding_pair(&name_str, true);
        let Some((_symbol, binding)) = binding else {
            std::println!("No binding {}", name_str);
            return String::new();
        };

        let ty = binding.type_id;
        let s = crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
            ty,
            dump_options(),
        );
        std::println!("{}", s);
        s
    }
}
