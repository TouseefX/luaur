//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyNormalizedTypes, L1006-1159)
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::functions::is_prim::is_prim;
use crate::functions::is_subclass_type::is_subclass_extern_type_extern_type;
use crate::functions::is_subtype_normalize::is_subtype_normalized_string_type_normalized_string_type;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::normalized_type::NormalizedType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimType};
use crate::records::singleton_type::SingletonType;
use crate::records::type_error::TypeError;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::sync::Arc;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Unifier {
    /// `void Unifier::tryUnifyNormalizedTypes(TypeId subTy, TypeId superTy, const NormalizedType& subNorm, const NormalizedType& superNorm, std::string reason, std::optional<TypeError> error)`
    pub fn unifier_try_unify_normalized_types(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        sub_norm: &NormalizedType,
        super_norm: &NormalizedType,
        reason: String,
        error: Option<TypeError>,
    ) {
        if !unsafe { get_type_id::<AnyType>(super_norm.tops) }.is_null() {
            return;
        } else if !unsafe { get_type_id::<AnyType>(sub_norm.tops) }.is_null() {
            self.failure = true;
            return;
        }

        if !unsafe { get_type_id::<ErrorType>(sub_norm.errors) }.is_null()
            && unsafe { get_type_id::<ErrorType>(super_norm.errors) }.is_null()
        {
            self.failure = true;
            return;
        }

        if !unsafe { get_type_id::<UnknownType>(super_norm.tops) }.is_null() {
            return;
        }

        if !unsafe { get_type_id::<UnknownType>(sub_norm.tops) }.is_null() {
            return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
        }

        if !unsafe { get_type_id::<PrimitiveType>(sub_norm.booleans) }.is_null() {
            if unsafe { get_type_id::<PrimitiveType>(super_norm.booleans) }.is_null() {
                return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
            }
        } else {
            let stv = unsafe { get_type_id::<SingletonType>(sub_norm.booleans) };
            if !stv.is_null()
                && unsafe { get_type_id::<PrimitiveType>(super_norm.booleans) }.is_null()
                && stv != unsafe { get_type_id::<SingletonType>(super_norm.booleans) }
            {
                return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
            }
        }

        if !unsafe { get_type_id::<PrimitiveType>(sub_norm.nils) }.is_null()
            && unsafe { get_type_id::<PrimitiveType>(super_norm.nils) }.is_null()
        {
            return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
        }

        if !unsafe { get_type_id::<PrimitiveType>(sub_norm.numbers) }.is_null()
            && unsafe { get_type_id::<PrimitiveType>(super_norm.numbers) }.is_null()
        {
            return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
        }

        if !is_subtype_normalized_string_type_normalized_string_type(
            &sub_norm.strings,
            &super_norm.strings,
        ) {
            return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
        }

        if !unsafe { get_type_id::<PrimitiveType>(sub_norm.threads) }.is_null()
            && unsafe { get_type_id::<PrimitiveType>(super_norm.errors) }.is_null()
        {
            return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
        }

        for (sub_extern_type, _) in sub_norm.extern_types.extern_types.iter() {
            let mut found = false;
            let sub_ctv = unsafe { get_type_id::<ExternType>(*sub_extern_type) };
            LUAU_ASSERT!(!sub_ctv.is_null());

            for (super_extern_type, super_negations) in super_norm.extern_types.extern_types.iter()
            {
                let super_ctv = unsafe { get_type_id::<ExternType>(*super_extern_type) };
                LUAU_ASSERT!(!super_ctv.is_null());

                if is_subclass_extern_type_extern_type(unsafe { &*sub_ctv }, unsafe { &*super_ctv })
                {
                    found = true;

                    for negation in super_negations.order.iter() {
                        let negation_ctv = unsafe { get_type_id::<ExternType>(*negation) };
                        LUAU_ASSERT!(!negation_ctv.is_null());

                        if is_subclass_extern_type_extern_type(unsafe { &*sub_ctv }, unsafe {
                            &*negation_ctv
                        }) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        break;
                    }
                }
            }

            if !found {
                return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
            }
        }

        let sub_tables = sub_norm.tables.order.clone();
        for sub_table in sub_tables {
            let mut found = false;
            let super_tables = super_norm.tables.order.clone();
            for super_table in super_tables {
                if is_prim(super_table, PrimType::Table) {
                    found = true;
                    break;
                }

                let mut inner_state = self.unifier_make_child_unifier();
                inner_state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                    sub_table,
                    super_table,
                    false,
                    false,
                    None,
                );

                if inner_state.errors.is_empty() {
                    found = true;
                    self.log.concat(inner_state.log);
                    break;
                } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                    return self.report_error_type_error(e);
                }
            }
            if !found {
                return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
            }
        }

        if !sub_norm.functions.is_never() {
            if super_norm.functions.is_never() {
                return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
            }
            let super_funs = super_norm.functions.parts.order.clone();
            for super_fun in super_funs {
                let mut inner_state = self.unifier_make_child_unifier();
                let super_ftv = unsafe { get_type_id::<FunctionType>(super_fun) };
                if super_ftv.is_null() {
                    return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
                }
                let tgt = inner_state.unifier_try_apply_overloaded_function(
                    sub_ty,
                    &sub_norm.functions,
                    unsafe { (*super_ftv).arg_types },
                );
                inner_state.try_unify_type_pack_id_type_pack_id_bool(
                    tgt,
                    unsafe { (*super_ftv).ret_types },
                    false,
                );
                if inner_state.errors.is_empty() {
                    self.log.concat(inner_state.log);
                } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                    return self.report_error_type_error(e);
                } else {
                    return self.report_normalized_mismatch(super_ty, sub_ty, reason, error);
                }
            }
        }

        let tyvars: alloc::vec::Vec<TypeId> = sub_norm.tyvars.keys().copied().collect();
        for tyvar in tyvars {
            let sub_intersect: &NormalizedType = sub_norm.tyvars.get(&tyvar).unwrap();
            match super_norm.tyvars.get(&tyvar) {
                None => self.unifier_try_unify_normalized_types(
                    sub_ty,
                    super_ty,
                    sub_intersect,
                    super_norm,
                    reason.clone(),
                    error.clone(),
                ),
                Some(found) => self.unifier_try_unify_normalized_types(
                    sub_ty,
                    super_ty,
                    sub_intersect,
                    found,
                    reason.clone(),
                    error.clone(),
                ),
            }
            if !self.errors.is_empty() {
                return;
            }
        }
    }

    /// `reportError(location, TypeMismatch{superTy, subTy, reason, error, mismatchContext()})`
    fn report_normalized_mismatch(
        &mut self,
        super_ty: TypeId,
        sub_ty: TypeId,
        reason: String,
        error: Option<TypeError>,
    ) {
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: super_ty,
                given_type: sub_ty,
                reason,
                error: error.map(Arc::new),
                context,
            }),
        );
    }
}
