use crate::functions::finite::finite;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::functions::size_type_pack::size;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::function_type::FunctionType;
use crate::records::instantiation::Instantiation;
use crate::records::type_error::TypeError;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use core::cmp::min;

impl Unifier {
    pub fn unifier_try_unify_functions(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        is_function_call: bool,
    ) {
        let mut super_function = unsafe { get_mutable_type_id::<FunctionType>(super_ty) };
        let mut sub_function = unsafe { get_mutable_type_id::<FunctionType>(sub_ty) };

        if super_function.is_null() || sub_function.is_null() {
            self.ice_string("passed non-function types to unifyFunction");
            return;
        }

        let mut num_generics = unsafe { (*super_function).generics.len() };
        let mut num_generic_packs = unsafe { (*super_function).generic_packs.len() };

        let should_instantiate = unsafe {
            (num_generics == 0 && !(*sub_function).generics.is_empty())
                || (num_generic_packs == 0 && !(*sub_function).generic_packs.is_empty())
        };

        if luaur_common::FFlag::LuauInstantiateInSubtyping.get() && should_instantiate {
            let mut instantiation = Instantiation::instantiation_new(
                &self.log as *const _,
                self.types,
                self.builtin_types,
                unsafe { (*self.scope).level },
                self.scope,
            );

            if let Some(instantiated) = instantiation.substitute_type_id(sub_ty) {
                sub_function = unsafe { get_mutable_type_id::<FunctionType>(instantiated) };
                if sub_function.is_null() {
                    self.ice_string(
                        "instantiation made a function type into a non-function type in unifyFunction",
                    );
                    return;
                }

                num_generics = min(unsafe { (*super_function).generics.len() }, unsafe {
                    (*sub_function).generics.len()
                });
                num_generic_packs = min(unsafe { (*super_function).generic_packs.len() }, unsafe {
                    (*sub_function).generic_packs.len()
                });
            } else {
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::UnificationTooComplex(
                        crate::records::unification_too_complex::UnificationTooComplex::default(),
                    ),
                );
            }
        } else if num_generics != unsafe { (*sub_function).generics.len() } {
            num_generics = min(num_generics, unsafe { (*sub_function).generics.len() });
            self.report_function_type_mismatch(
                super_ty,
                sub_ty,
                "different number of generic type parameters",
                None,
            );
        }

        if num_generic_packs != unsafe { (*sub_function).generic_packs.len() } {
            num_generic_packs = min(num_generic_packs, unsafe {
                (*sub_function).generic_packs.len()
            });
            self.report_function_type_mismatch(
                super_ty,
                sub_ty,
                "different number of generic type pack parameters",
                None,
            );
        }

        for i in 0..num_generics {
            unsafe {
                let super_generics = &(*super_function).generics;
                let sub_generics = &(*sub_function).generics;
                self.log
                    .push_seen_type_id_type_id(super_generics[i], sub_generics[i]);
            }
        }

        for i in 0..num_generic_packs {
            unsafe {
                let super_generic_packs = &(*super_function).generic_packs;
                let sub_generic_packs = &(*sub_function).generic_packs;
                self.log.push_seen_type_pack_id_type_pack_id(
                    super_generic_packs[i],
                    sub_generic_packs[i],
                );
            }
        }

        let context = self.ctx;

        if !is_function_call {
            let mut inner_state = self.unifier_make_child_unifier();

            inner_state.ctx = CountMismatchContext::Arg;
            unsafe {
                inner_state.try_unify_type_pack_id_type_pack_id_bool(
                    (*super_function).arg_types,
                    (*sub_function).arg_types,
                    is_function_call,
                );
            }

            let reported = !inner_state.errors.is_empty();

            if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                self.report_error_type_error(e);
            } else if !inner_state.errors.is_empty() && inner_state.first_pack_error_pos.is_some() {
                let reason = format!(
                    "Argument #{} type is not compatible.",
                    inner_state.first_pack_error_pos.unwrap()
                );
                self.report_function_type_mismatch(
                    super_ty,
                    sub_ty,
                    &reason,
                    inner_state.errors.first().cloned(),
                );
            } else if !inner_state.errors.is_empty() {
                self.report_function_type_mismatch(
                    super_ty,
                    sub_ty,
                    "",
                    inner_state.errors.first().cloned(),
                );
            }

            inner_state.ctx = CountMismatchContext::FunctionResult;
            unsafe {
                inner_state.try_unify_type_pack_id_type_pack_id_bool(
                    (*sub_function).ret_types,
                    (*super_function).ret_types,
                    false,
                );
            }

            if !reported {
                if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                    self.report_error_type_error(e);
                } else if !inner_state.errors.is_empty()
                    && unsafe { size((*super_function).ret_types, core::ptr::null_mut()) == 1 }
                    && unsafe { finite((*super_function).ret_types, core::ptr::null_mut()) }
                {
                    self.report_function_type_mismatch(
                        super_ty,
                        sub_ty,
                        "Return type is not compatible.",
                        inner_state.errors.first().cloned(),
                    );
                } else if !inner_state.errors.is_empty()
                    && inner_state.first_pack_error_pos.is_some()
                {
                    let reason = format!(
                        "Return #{} type is not compatible.",
                        inner_state.first_pack_error_pos.unwrap()
                    );
                    self.report_function_type_mismatch(
                        super_ty,
                        sub_ty,
                        &reason,
                        inner_state.errors.first().cloned(),
                    );
                } else if !inner_state.errors.is_empty() {
                    self.report_function_type_mismatch(
                        super_ty,
                        sub_ty,
                        "",
                        inner_state.errors.first().cloned(),
                    );
                }
            }

            self.log.concat(inner_state.log);
        } else {
            self.ctx = CountMismatchContext::Arg;
            unsafe {
                self.try_unify_type_pack_id_type_pack_id_bool(
                    (*super_function).arg_types,
                    (*sub_function).arg_types,
                    is_function_call,
                );
            }

            self.ctx = CountMismatchContext::FunctionResult;
            unsafe {
                self.try_unify_type_pack_id_type_pack_id_bool(
                    (*sub_function).ret_types,
                    (*super_function).ret_types,
                    false,
                );
            }
        }

        super_function = unsafe { get_mutable_type_id::<FunctionType>(super_ty) };
        sub_function = unsafe { get_mutable_type_id::<FunctionType>(sub_ty) };

        self.ctx = context;

        for i in (0..num_generic_packs).rev() {
            unsafe {
                let super_generic_packs = &(*super_function).generic_packs;
                let sub_generic_packs = &(*sub_function).generic_packs;
                self.log.pop_seen_type_pack_id_type_pack_id(
                    super_generic_packs[i],
                    sub_generic_packs[i],
                );
            }
        }

        for i in (0..num_generics).rev() {
            unsafe {
                let super_generics = &(*super_function).generics;
                let sub_generics = &(*sub_function).generics;
                self.log
                    .pop_seen_type_id_type_id(super_generics[i], sub_generics[i]);
            }
        }
    }

    fn report_function_type_mismatch(
        &mut self,
        super_ty: TypeId,
        sub_ty: TypeId,
        reason: &str,
        err: Option<TypeError>,
    ) {
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: super_ty,
                given_type: sub_ty,
                reason: String::from(reason),
                error: err.map(Arc::new),
                context,
            }),
        );
    }
}
