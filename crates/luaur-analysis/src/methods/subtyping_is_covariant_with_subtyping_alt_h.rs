use crate::enums::type_field::TypeField;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::error_type::ErrorType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_negation: &NegationType,
        super_ty: TypeId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let negated_ty = unsafe { follow_type_id(sub_negation.ty) };

        let mut result = SubtypingResult::default();

        if unsafe { !get_type_id::<NeverType>(negated_ty).is_null() } {
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                unsafe { (*self.builtin_types).unknownType },
                super_ty,
                scope,
            );
            result.with_sub_component(Component::TypeField(TypeField::Negated));
        } else if unsafe { !get_type_id::<UnknownType>(negated_ty).is_null() } {
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                unsafe { (*self.builtin_types).neverType },
                super_ty,
                scope,
            );
            result.with_sub_component(Component::TypeField(TypeField::Negated));
        } else if unsafe { !get_type_id::<AnyType>(negated_ty).is_null() } {
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env, negated_ty, super_ty, scope,
            );
            result.with_sub_component(Component::TypeField(TypeField::Negated));
        } else if let Some(u) = unsafe { get_type_id::<UnionType>(negated_ty).as_ref() } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };

            for ty in &u.options {
                if let Some(negated_part) =
                    unsafe { get_type_id::<NegationType>(follow_type_id(*ty)).as_ref() }
                {
                    let mut inner = self
                        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                            env,
                            negated_part.ty,
                            super_ty,
                            scope,
                        );
                    inner.with_sub_component(Component::TypeField(TypeField::Negated));
                    result.and_also(
                        inner,
                        crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
                    );
                } else {
                    let negated_tmp = NegationType { ty: *ty };
                    result.and_also(
                        self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
                            env,
                            &negated_tmp,
                            super_ty,
                            scope,
                        ),
                        crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
                    );
                }
            }
        } else if let Some(i) = unsafe { get_type_id::<IntersectionType>(negated_ty).as_ref() } {
            result = SubtypingResult {
                is_subtype: false,
                ..Default::default()
            };

            for ty in &i.parts {
                if let Some(negated_part) =
                    unsafe { get_type_id::<NegationType>(follow_type_id(*ty)).as_ref() }
                {
                    let mut inner = self
                        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                            env,
                            negated_part.ty,
                            super_ty,
                            scope,
                        );
                    inner.with_sub_component(Component::TypeField(TypeField::Negated));
                    result.or_else(inner);
                } else {
                    let negated_tmp = NegationType { ty: *ty };
                    result.or_else(
                        self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
                            env,
                            &negated_tmp,
                            super_ty,
                            scope,
                        ),
                    );
                }
            }
        } else if unsafe {
            !get_type_id::<ErrorType>(negated_ty).is_null()
                || !get_type_id::<FunctionType>(negated_ty).is_null()
                || !get_type_id::<TableType>(negated_ty).is_null()
                || !get_type_id::<MetatableType>(negated_ty).is_null()
        } {
            unsafe {
                (*self.ice_reporter).ice_string("attempting to negate a non-testable type");
            }
        } else {
            result = SubtypingResult {
                is_subtype: false,
                ..Default::default()
            };
            result.with_sub_component(Component::TypeField(TypeField::Negated));
        }

        result
    }
}
