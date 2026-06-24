use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::records::any_type::AnyType;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::sync::Arc;

impl Unifier {
    pub fn unifier_try_unify_with_metatable(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        reversed: bool,
    ) {
        let super_metatable = unsafe { get_type_id::<MetatableType>(super_ty) };
        if super_metatable.is_null() {
            self.ice_string("tryUnifyMetatable invoked with non-metatable Type");
            return;
        }

        let wanted = if reversed { sub_ty } else { super_ty };
        let given = if reversed { super_ty } else { sub_ty };

        if let Some(sub_metatable) =
            unsafe { get_mutable_type_id::<MetatableType>(sub_ty).as_mut() }
        {
            let mut inner_state = self.unifier_make_child_unifier();
            inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_metatable.table,
                unsafe { (*super_metatable).table },
                false,
                false,
                None,
            );
            inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_metatable.metatable,
                unsafe { (*super_metatable).metatable },
                false,
                false,
                None,
            );

            if let Some(e) = has_unification_too_complex(&inner_state.errors) {
                self.report_error_type_error(e);
            } else if !inner_state.errors.is_empty() {
                let context = self.unifier_mismatch_context();
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::TypeMismatch(TypeMismatch {
                        wanted_type: wanted,
                        given_type: given,
                        reason: String::new(),
                        error: Some(Arc::new(inner_state.errors[0].clone())),
                        context,
                    }),
                );
            }

            self.log.concat(inner_state.log);
            self.failure |= inner_state.failure;
        } else if let Some(sub_table) = unsafe { get_mutable_type_id::<TableType>(sub_ty).as_mut() }
        {
            match sub_table.state {
                crate::enums::table_state::TableState::Free => {
                    self.try_unify_type_id_type_id_bool_bool_literal_properties(
                        sub_ty,
                        unsafe { (*super_metatable).table },
                        false,
                        false,
                        None,
                    );
                    self.log.bind_table(sub_ty, Some(super_ty));
                }
                crate::enums::table_state::TableState::Sealed
                | crate::enums::table_state::TableState::Unsealed
                | crate::enums::table_state::TableState::Generic => {
                    let context = self.unifier_mismatch_context();
                    self.report_error_location_type_error_data(
                        self.location,
                        TypeErrorData::TypeMismatch(TypeMismatch {
                            wanted_type: wanted,
                            given_type: given,
                            reason: String::new(),
                            error: None,
                            context,
                        }),
                    );
                }
            }
        } else if !unsafe { get_mutable_type_id::<AnyType>(sub_ty) }.is_null()
            || !unsafe { get_mutable_type_id::<ErrorType>(sub_ty) }.is_null()
        {
        } else {
            let context = self.unifier_mismatch_context();
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::TypeMismatch(TypeMismatch {
                    wanted_type: wanted,
                    given_type: given,
                    reason: String::new(),
                    error: None,
                    context,
                }),
            );
        }
    }
}
