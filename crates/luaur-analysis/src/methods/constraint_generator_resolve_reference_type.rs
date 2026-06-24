use crate::enums::polarity::Polarity;
use crate::functions::finite::finite;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::size_type_pack::size;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generic_error::GenericError;
use crate::records::generic_type::GenericType;
use crate::records::module::Module;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::scope::Scope;
use crate::records::type_alias_expansion_constraint::TypeAliasExpansionConstraint;
use crate::records::type_fun::TypeFun;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::unapplied_type_function::UnappliedTypeFunction;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn resolve_reference_type(
        &mut self,
        scope: &ScopePtr,
        ty: *mut AstType,
        ref_: *mut AstTypeReference,
        in_type_arguments: bool,
        replace_error_with_fresh: bool,
    ) -> TypeId {
        let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
        let mut result: TypeId = core::ptr::null_mut();

        if FFlag::DebugLuauMagicTypes.get() {
            let ref_name_str = unsafe {
                core::ffi::CStr::from_ptr((*ref_).name.value)
                    .to_string_lossy()
                    .into_owned()
            };

            if ref_name_str == "_luau_ice" {
                let location = unsafe { (*ty).base.location };
                unsafe { (*self.ice).ice_string_location("_luau_ice encountered", &location) };
            } else if ref_name_str == "_luau_print" {
                let location = unsafe { (*ty).base.location };
                let params = unsafe { &(*ref_).parameters };
                if params.size != 1 || unsafe { (*params.data.add(0)).r#type }.is_null() {
                    let err = GenericError::new(alloc::string::String::from(
                        "_luau_print requires one generic parameter",
                    ));
                    self.report_error(location, TypeErrorData::GenericError(err));
                    unsafe {
                        let module_ptr =
                            alloc::sync::Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
                        *(*module_ptr)
                            .ast_resolved_types
                            .get_or_insert(ty as *const _) = (*self.builtin_types).errorType;
                        return (*self.builtin_types).errorType;
                    }
                } else {
                    let param_ty = unsafe { (*params.data.add(0)).r#type };
                    return self.resolve_type_constraint_generator_alt_b(
                        scope_ptr,
                        param_ty,
                        in_type_arguments,
                        false,
                    );
                }
            } else if ref_name_str == "_luau_blocked_type" {
                return unsafe { (*self.arena).add_type(BlockedType::default()) };
            }
        }

        let mut alias: Option<TypeFun> = None;

        let name_str = unsafe {
            core::ffi::CStr::from_ptr((*ref_).name.value)
                .to_string_lossy()
                .into_owned()
        };
        if let Some(prefix_name) = unsafe { (*ref_).prefix } {
            let prefix_str = unsafe {
                core::ffi::CStr::from_ptr(prefix_name.value)
                    .to_string_lossy()
                    .into_owned()
            };
            alias = scope.lookup_imported_type(&prefix_str, &name_str);
        } else {
            alias = scope.lookup_type(&name_str);
        }

        if let Some(ref alias_ref) = alias {
            if !alias_ref.type_params.is_empty()
                || !alias_ref.type_pack_params.is_empty()
                || unsafe { (*ref_).has_parameter_list }
            {
                let mut parameters: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
                let mut pack_parameters: alloc::vec::Vec<TypePackId> = alloc::vec::Vec::new();

                let params_array = unsafe { &(*ref_).parameters };
                for i in 0..params_array.size {
                    let param = unsafe { *params_array.data.add(i) };
                    if !param.r#type.is_null() {
                        let param_ty = self.resolve_type_constraint_generator_alt_b(
                            scope_ptr,
                            param.r#type,
                            true,
                            false,
                        );
                        parameters.push(param_ty);
                    } else if !param.type_pack.is_null() {
                        let tp = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
                            scope_ptr,
                            param.type_pack,
                            true,
                            false,
                        );

                        // If we need more regular typeArguments, we can use single
                        // element type packs to fill those in.
                        if parameters.len() < alias_ref.type_params.len()
                            && size(tp, core::ptr::null_mut()) == 1
                            && finite(tp, core::ptr::null_mut())
                        {
                            if let Some(ty_val) = first(tp, false) {
                                parameters.push(ty_val);
                                continue;
                            }
                        }
                        pack_parameters.push(tp);
                    } else {
                        LUAU_ASSERT!(false);
                    }
                }

                let pending = PendingExpansionType::pending_expansion_type_pending_expansion_type(
                    unsafe { (*ref_).prefix },
                    unsafe { (*ref_).name },
                    parameters,
                    pack_parameters,
                );
                result = unsafe { (*self.arena).add_type(pending) };

                if !in_type_arguments {
                    let location = unsafe { (*ty).base.location };
                    let constraint = TypeAliasExpansionConstraint { target: result };
                    self.add_constraint_scope_ptr_location_constraint_v(
                        scope,
                        location,
                        ConstraintV::TypeAliasExpansion(constraint),
                    );
                }
            } else {
                result = alias_ref.r#type();
            }
        } else {
            result = unsafe { (*self.builtin_types).errorType };
            if replace_error_with_fresh {
                result = self.fresh_type(scope, Polarity::Mixed);
            }
        }

        let follow_result = unsafe { follow_type_id(result) };
        if !unsafe { get_type_id::<TypeFunctionInstanceType>(follow_result) }.is_null() {
            let location = unsafe { (*ty).base.location };
            self.report_error(
                location,
                TypeErrorData::UnappliedTypeFunction(UnappliedTypeFunction::default()),
            );
            let reduce_constraint = ReduceConstraint { ty: result };
            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                location,
                ConstraintV::Reduce(reduce_constraint),
            );
        }

        let follow_result = unsafe { follow_type_id(result) };
        let generic_type = unsafe { getMutable::<GenericType>(follow_result) };
        if !generic_type.is_null() {
            let current_polarity = unsafe { (*generic_type).polarity };
            unsafe {
                (*generic_type).polarity = (current_polarity & Polarity::Mixed) | self.polarity;
            }
        }

        result
    }
}
