use crate::records::duplicate_generic_parameter::DuplicateGenericParameter;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type::AstGenericType;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl NonStrictTypeChecker {
    pub fn visit_generics(
        &mut self,
        generics: AstArray<*mut AstGenericType>,
        generic_packs: AstArray<*mut AstGenericTypePack>,
    ) {
        // C++ `DenseHashSet<AstName> seen{AstName{}};` — empty/null AstName sentinel key.
        let mut seen: DenseHashSet<AstName> = DenseHashSet::new(AstName {
            value: core::ptr::null(),
        });

        let mut i = 0;
        while i < generics.size {
            let g = unsafe { *generics.data.add(i) };
            let name = unsafe { (*g).name };

            if seen.contains(&name) {
                let param_name = unsafe {
                    core::ffi::CStr::from_ptr(name.value)
                        .to_string_lossy()
                        .into_owned()
                };
                self.report_error(
                    TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(
                        param_name,
                    )),
                    &unsafe { (*g).base.location },
                );
            } else {
                seen.insert(name);
            }

            let default_value = unsafe { (*g).default_value };
            if !default_value.is_null() {
                self.visit_ast_type(default_value);
            }

            i += 1;
        }

        let mut j = 0;
        while j < generic_packs.size {
            let g = unsafe { *generic_packs.data.add(j) };
            let name = unsafe { (*g).name };

            if seen.contains(&name) {
                let param_name = unsafe {
                    core::ffi::CStr::from_ptr(name.value)
                        .to_string_lossy()
                        .into_owned()
                };
                self.report_error(
                    TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(
                        param_name,
                    )),
                    &unsafe { (*g).base.location },
                );
            } else {
                seen.insert(name);
            }

            let default_value = unsafe { (*g).default_value };
            if !default_value.is_null() {
                self.visit_ast_type_pack(default_value);
            }

            j += 1;
        }
    }
}
