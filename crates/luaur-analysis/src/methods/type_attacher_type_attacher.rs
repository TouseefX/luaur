impl crate::records::type_attacher::TypeAttacher {
    pub fn type_attacher_type_attacher(
        checker: *mut crate::records::module::Module,
        alloc: *mut luaur_ast::records::allocator::Allocator,
    ) -> Self {
        Self {
            module: checker,
            allocator: alloc,
            // C++ default-constructs `SyntheticNames syntheticNames;`; the Rust
            // DenseHashMap uses a null-pointer empty-key sentinel.
            synthetic_names: crate::type_aliases::synthetic_names::SyntheticNames::new(
                core::ptr::null_mut(),
            ),
        }
    }
}
