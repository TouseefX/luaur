use crate::records::contains_generics::ContainsGenerics;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ContainsGenerics {
    pub fn contains_generics_contains_generics(generics: *mut DenseHashSet<*const c_void>) -> Self {
        let mut visitor = ContainsGenerics {
            base: IterativeTypeVisitor {
                seen: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null_mut(),
                ),
                work_queue: alloc::vec::Vec::new(),
                parent_cursor: -1,
                work_cursor: 0,
                visitor_name: alloc::string::String::from("ContainsGenerics"),
                skip_bound_types: true,
                visit_once: true,
            },
            generics,
            found: false,
        };
        visitor
    }
}
