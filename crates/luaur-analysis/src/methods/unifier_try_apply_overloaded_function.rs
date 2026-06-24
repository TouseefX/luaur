//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryApplyOverloadedFunction, L1161-1224)
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

impl Unifier {
    /// `TypePackId Unifier::tryApplyOverloadedFunction(TypeId function, const NormalizedFunctionType& overloads, TypePackId args)`
    pub fn unifier_try_apply_overloaded_function(
        &mut self,
        function: TypeId,
        overloads: &NormalizedFunctionType,
        args: TypePackId,
    ) -> TypePackId {
        if overloads.is_never() {
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: function }),
            );
            return unsafe { (*self.builtin_types).errorTypePack };
        }

        let mut result: Option<TypePackId> = None;
        let mut first_fun: *const FunctionType = core::ptr::null();

        let parts = overloads.parts.order.clone();
        for overload in parts {
            let ftv = unsafe { get_type_id::<FunctionType>(overload) };
            if !ftv.is_null() {
                // TODO: instantiate generics?
                if unsafe { (*ftv).generics.is_empty() && (*ftv).generic_packs.is_empty() } {
                    if first_fun.is_null() {
                        first_fun = ftv;
                    }
                    let mut inner_state = self.unifier_make_child_unifier();
                    inner_state.try_unify_type_pack_id_type_pack_id_bool(
                        args,
                        unsafe { (*ftv).arg_types },
                        false,
                    );
                    if inner_state.errors.is_empty() {
                        self.log.concat(inner_state.log.clone());
                        if let Some(res) = result {
                            inner_state.log.clear();
                            inner_state.try_unify_type_pack_id_type_pack_id_bool(
                                res,
                                unsafe { (*ftv).ret_types },
                                false,
                            );
                            if inner_state.errors.is_empty() {
                                self.log.concat(inner_state.log.clone());
                            }
                            // Annoyingly, since we don't support intersection of generic type packs,
                            // the intersection may fail. We rather arbitrarily use the first matching overload
                            // in that case.
                            else if let Some(intersect) = unsafe {
                                (*self.normalizer).intersection_of_type_packs(res, (*ftv).ret_types)
                            } {
                                result = Some(intersect);
                            }
                        } else {
                            result = Some(unsafe { (*ftv).ret_types });
                        }
                    } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                        self.report_error_type_error(e);
                        return unsafe { (*self.builtin_types).error_recovery_type_pack(args) };
                    }
                }
            }
        }

        if let Some(res) = result {
            res
        } else if !first_fun.is_null() {
            // TODO: better error reporting?
            // The logic for error reporting overload resolution
            // is currently over in TypeInfer.cpp, should we move it?
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::GenericError(GenericError::new(String::from(
                    "No matching overload.",
                ))),
            );
            unsafe { (*self.builtin_types).error_recovery_type_pack((*first_fun).ret_types) }
        } else {
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: function }),
            );
            unsafe { (*self.builtin_types).errorTypePack }
        }
    }
}
