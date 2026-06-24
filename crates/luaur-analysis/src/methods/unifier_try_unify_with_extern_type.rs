use crate::enums::table_state::TableState;
use crate::enums::variance::Variance;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_subclass_type::is_subclass_extern_type_extern_type;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::records::extern_type::ExternType;
use crate::records::generic_error::GenericError;
use crate::records::table_type::TableType;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;

impl Unifier {
    pub fn unifier_try_unify_with_extern_type(
        &mut self,
        mut sub_ty: TypeId,
        mut super_ty: TypeId,
        reversed: bool,
    ) {
        if reversed {
            core::mem::swap(&mut super_ty, &mut sub_ty);
        }

        let super_extern_type = unsafe { get_type_id::<ExternType>(super_ty) };
        if super_extern_type.is_null() {
            self.ice_string("tryUnifyExternType invoked with non-class Type");
            return;
        }

        if let Some(sub_extern_type) = unsafe { get_type_id::<ExternType>(sub_ty).as_ref() } {
            match self.variance {
                Variance::Covariant => {
                    if !is_subclass_extern_type_extern_type(sub_extern_type, unsafe {
                        &*super_extern_type
                    }) {
                        self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
                    }
                    return;
                }
                Variance::Invariant => {
                    if sub_extern_type as *const ExternType != super_extern_type {
                        self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
                    }
                    return;
                }
            }
        } else if let Some(sub_table) = unsafe { get_mutable_type_id::<TableType>(sub_ty).as_mut() }
        {
            if sub_table.state != TableState::Free {
                self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
                return;
            }

            let mut ok = true;

            for (prop_name, prop) in sub_table.props.iter() {
                let class_prop = lookup_extern_type_prop(unsafe { &*super_extern_type }, prop_name);
                if class_prop.is_null() {
                    ok = false;
                    self.report_error_location_type_error_data(
                        self.location,
                        TypeErrorData::UnknownProperty(UnknownProperty {
                            table: super_ty,
                            key: prop_name.clone(),
                        }),
                    );
                } else {
                    let mut inner_state = self.unifier_make_child_unifier();
                    let class_prop_ty = unsafe {
                        (*class_prop)
                            .read_ty
                            .or((*class_prop).write_ty)
                            .unwrap_or(core::ptr::null())
                    };
                    let prop_ty = prop.read_ty.or(prop.write_ty).unwrap_or(core::ptr::null());
                    inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                        class_prop_ty,
                        prop_ty,
                        false,
                        false,
                        None,
                    );

                    self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                        &inner_state.errors,
                        prop_name,
                        if reversed { sub_ty } else { super_ty },
                        if reversed { super_ty } else { sub_ty },
                    );

                    if inner_state.errors.is_empty() {
                        self.log.concat(inner_state.log);
                        self.failure |= inner_state.failure;
                    } else {
                        ok = false;
                    }
                }
            }

            if sub_table.indexer.is_some() {
                ok = false;
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::GenericError(GenericError::new(format!(
                        "Extern type {} does not have an indexer",
                        unsafe { &*super_extern_type }.name
                    ))),
                );
            }

            if !ok {
                return;
            }

            self.log.bind_table(sub_ty, Some(super_ty));
        } else {
            self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
        }
    }

    fn report_extern_type_mismatch(&mut self, sub_ty: TypeId, super_ty: TypeId, reversed: bool) {
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: if reversed { sub_ty } else { super_ty },
                given_type: if reversed { super_ty } else { sub_ty },
                reason: String::new(),
                error: None,
                context,
            }),
        );
    }
}
