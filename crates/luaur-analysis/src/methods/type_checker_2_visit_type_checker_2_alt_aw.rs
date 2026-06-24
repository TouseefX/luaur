//! Faithful port of `TypeChecker2::visit(AstTypeReference*)` (TypeChecker2.cpp:2800-2938).
use crate::functions::finite::finite;
use crate::functions::first::first;
use crate::functions::size_type_pack::size;
use crate::records::generic_error::GenericError;
use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;
use crate::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_fun::TypeFun;
use crate::records::unknown_symbol::Context;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::name_type::Name;
use alloc::string::String;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn visit_ast_type_reference(&mut self, ty: *mut AstTypeReference) {
        let ty_ref = unsafe { &*ty };
        let location = ty_ref.base.base.location;

        // No further validation is necessary in this case. The main logic for
        // _luau_print is contained in lookupAnnotation.
        // C++: kLuauPrint, kLuauForceConstraintSolvingIncomplete, kLuauBlockedType.
        if FFlag::DebugLuauMagicTypes.get() {
            let magic_name =
                unsafe { core::ffi::CStr::from_ptr(ty_ref.name.value).to_string_lossy() };
            if magic_name == "_luau_print"
                || magic_name == "_luau_force_constraint_solving_incomplete"
                || magic_name == "_luau_blocked_type"
            {
                return;
            }
        }

        let params = ty_ref.parameters;
        for i in 0..params.size {
            let param = unsafe { *params.data.add(i) };
            if !param.r#type.is_null() {
                self.visit_ast_type(param.r#type);
            } else {
                self.visit_ast_type_pack(param.type_pack);
            }
        }

        let scope_ptr = self.find_innermost_scope(location);
        LUAU_ASSERT!(!scope_ptr.is_null());
        let scope = unsafe { &*scope_ptr };

        let name_str: Name = unsafe {
            core::ffi::CStr::from_ptr(ty_ref.name.value)
                .to_string_lossy()
                .into_owned()
        };

        let alias: Option<TypeFun> = if let Some(prefix) = ty_ref.prefix {
            let prefix_str: Name = unsafe {
                core::ffi::CStr::from_ptr(prefix.value)
                    .to_string_lossy()
                    .into_owned()
            };
            scope.lookup_imported_type(&prefix_str, &name_str)
        } else {
            scope.lookup_type(&name_str)
        };

        if let Some(ref alias_ref) = alias {
            let types_required = alias_ref.type_params().len();
            let packs_required = alias_ref.type_pack_params().len();

            let mut types_provided = 0usize;
            let mut extra_types = 0usize;
            let mut packs_provided = 0usize;

            for i in 0..params.size {
                let param = unsafe { *params.data.add(i) };
                if !param.r#type.is_null() {
                    if packs_provided != 0 {
                        self.report_error_type_error_data_location(
                            GenericError::new(String::from(
                                "Type parameters must come before type pack parameters",
                            ))
                            .into(),
                            &location,
                        );
                        continue;
                    }

                    if types_provided < types_required {
                        types_provided += 1;
                    } else {
                        extra_types += 1;
                    }
                } else if !param.type_pack.is_null() {
                    let tp = self.lookup_pack_annotation(param.type_pack);
                    if tp.is_none() {
                        continue;
                    }

                    let tp_id = tp.unwrap();
                    // C++ `size(*tp) == 1 && finite(*tp) && first(*tp)`.
                    if types_provided < types_required
                        && size(tp_id, core::ptr::null_mut()) == 1
                        && finite(tp_id, core::ptr::null_mut())
                        && first(tp_id, true).is_some()
                    {
                        types_provided += 1;
                    } else {
                        packs_provided += 1;
                    }
                }
            }

            // If we require type parameters, but no typeArguments are provided and only packs are provided, we report an error.
            if types_required != 0 && types_provided == 0 && packs_provided != 0 {
                self.report_error_type_error_data_location(
                    GenericError::new(String::from(
                        "Type parameters must come before type pack parameters",
                    ))
                    .into(),
                    &location,
                );
            }

            if extra_types != 0 && packs_provided == 0 {
                // Extra typeArguments are only collected into a pack if a pack is expected
                if packs_required != 0 {
                    packs_provided += 1;
                } else {
                    types_provided += extra_types;
                }
            }

            for idx in types_provided..types_required {
                if alias_ref.type_params()[idx].defaultValue.is_some() {
                    types_provided += 1;
                }
            }

            for idx in packs_provided..packs_required {
                if alias_ref.type_pack_params()[idx].defaultValue.is_some() {
                    packs_provided += 1;
                }
            }

            // If the type parameter list is explicitly provided, allow an empty type pack to satisfy the expected pack count.
            if extra_types == 0 && packs_provided + 1 == packs_required && ty_ref.has_parameter_list
            {
                packs_provided += 1;
            }

            if types_provided != types_required || packs_provided != packs_required {
                self.report_error_type_error_data_location(
                    IncorrectGenericParameterCount {
                        name: name_str.clone(),
                        type_fun: alias_ref.clone(),
                        actual_parameters: types_provided,
                        actual_pack_parameters: packs_provided,
                    }
                    .into(),
                    &location,
                );
            }
        } else if scope.lookup_pack(&name_str).is_some() {
            self.report_error_type_error_data_location(
                SwappedGenericTypeParameter {
                    name: String::from(unsafe {
                        core::ffi::CStr::from_ptr(ty_ref.name.value).to_string_lossy()
                    }),
                    kind: SwappedGenericTypeParameter::Type,
                }
                .into(),
                &location,
            );
        } else {
            let mut symbol = String::new();
            if let Some(prefix) = ty_ref.prefix {
                let prefix_str = unsafe {
                    core::ffi::CStr::from_ptr(prefix.value)
                        .to_string_lossy()
                        .into_owned()
                };
                symbol.push_str(&prefix_str);
                symbol.push('.');
            }
            symbol.push_str(unsafe {
                core::ffi::CStr::from_ptr(ty_ref.name.value)
                    .to_string_lossy()
                    .as_ref()
            });

            self.report_error_type_error_data_location(
                UnknownSymbol::new(symbol, Context::Type).into(),
                &location,
            );
        }
    }
}
