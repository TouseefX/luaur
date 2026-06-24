use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::enums::type_field::TypeField;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_2::get2;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::error_type::ErrorType;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimType};
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: TypeId,
        super_negation: &NegationType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let negated_ty = unsafe { follow_type_id(super_negation.ty) };

        let mut result = SubtypingResult::default();

        if unsafe { !get_type_id::<NeverType>(negated_ty).is_null() } {
            // ¬never ~ unknown
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_ty,
                unsafe { (*self.builtin_types).unknownType },
                scope,
            );
        } else if unsafe { !get_type_id::<UnknownType>(negated_ty).is_null() } {
            // ¬unknown ~ never
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_ty,
                unsafe { (*self.builtin_types).neverType },
                scope,
            );
        } else if unsafe { !get_type_id::<AnyType>(negated_ty).is_null() } {
            // ¬any ~ any
            result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env, sub_ty, negated_ty, scope,
            );
        } else if let Some(u) = unsafe { get_type_id::<UnionType>(negated_ty).as_ref() } {
            // ¬(A ∪ B) ~ ¬A ∩ ¬B
            // follow intersection rules: A & B <: T iff A <: T && B <: T
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };

            for ty in &u.options {
                if let Some(negated_part) =
                    unsafe { get_type_id::<NegationType>(follow_type_id(*ty)).as_ref() }
                {
                    result.and_also(
                        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                            env, sub_ty, negated_part.ty, scope,
                        ),
                        SubtypingSuppressionPolicy::Any,
                    );
                } else {
                    let negated_tmp = NegationType { ty: *ty };
                    result.and_also(
                        self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
                            env, sub_ty, &negated_tmp, scope,
                        ),
                        SubtypingSuppressionPolicy::Any,
                    );
                }
            }
        } else if let Some(i) = unsafe { get_type_id::<IntersectionType>(negated_ty).as_ref() } {
            // ¬(A ∩ B) ~ ¬A ∪ ¬B
            // follow union rules: A | B <: T iff A <: T || B <: T
            result = SubtypingResult {
                is_subtype: false,
                ..Default::default()
            };

            for ty in &i.parts {
                if let Some(negated_part) =
                    unsafe { get_type_id::<NegationType>(follow_type_id(*ty)).as_ref() }
                {
                    result.or_else(
                        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                            env, sub_ty, negated_part.ty, scope,
                        ),
                    );
                } else {
                    let negated_tmp = NegationType { ty: *ty };
                    result.or_else(
                        self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
                            env, sub_ty, &negated_tmp, scope,
                        ),
                    );
                }
            }
        } else if {
            let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            // number <: ¬boolean
            // number </: ¬number
            let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, negated_ty);
            result = SubtypingResult {
                is_subtype: unsafe { (*p.first).r#type != (*p.second).r#type },
                ..Default::default()
            };
        } else if {
            let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, negated_ty);
            // "foo" </: ¬string
            if unsafe {
                (*p.first).variant.get_if::<StringSingleton>().is_some()
                    && (*p.second).r#type == PrimType::String
            } {
                result = SubtypingResult {
                    is_subtype: false,
                    ..Default::default()
                };
            }
            // false </: ¬boolean
            else if unsafe {
                (*p.first).variant.get_if::<BooleanSingleton>().is_some()
                    && (*p.second).r#type == PrimType::Boolean
            } {
                result = SubtypingResult {
                    is_subtype: false,
                    ..Default::default()
                };
            }
            // other cases are true
            else {
                result = SubtypingResult {
                    is_subtype: true,
                    ..Default::default()
                };
            }
        } else if {
            let p = get2::<PrimitiveType, SingletonType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            let p = get2::<PrimitiveType, SingletonType, _>(sub_ty, negated_ty);
            if unsafe {
                (*p.first).r#type == PrimType::String
                    && (*p.second).variant.get_if::<StringSingleton>().is_some()
            } {
                result = SubtypingResult {
                    is_subtype: false,
                    ..Default::default()
                };
            } else if unsafe {
                (*p.first).r#type == PrimType::Boolean
                    && (*p.second).variant.get_if::<BooleanSingleton>().is_some()
            } {
                result = SubtypingResult {
                    is_subtype: false,
                    ..Default::default()
                };
            } else {
                result = SubtypingResult {
                    is_subtype: true,
                    ..Default::default()
                };
            }
        }
        // the top class type is not actually a primitive type, so the negation of
        // any one of them includes the top class type.
        else if {
            let p = get2::<ExternType, PrimitiveType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        } else if unsafe {
            let p = get_type_id::<PrimitiveType>(negated_ty);
            !p.is_null()
                && (!get_type_id::<TableType>(sub_ty).is_null()
                    || !get_type_id::<MetatableType>(sub_ty).is_null())
        } {
            let p = unsafe { get_type_id::<PrimitiveType>(negated_ty) };
            result = SubtypingResult {
                is_subtype: unsafe { (*p).r#type != PrimType::Table },
                ..Default::default()
            };
        } else if {
            let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, negated_ty);
            result = SubtypingResult {
                is_subtype: unsafe { (*p.second).r#type != PrimType::Function },
                ..Default::default()
            };
        } else if {
            let p = get2::<SingletonType, SingletonType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            let p = get2::<SingletonType, SingletonType, _>(sub_ty, negated_ty);
            result = SubtypingResult {
                is_subtype: unsafe { *p.first != *p.second },
                ..Default::default()
            };
        } else if {
            let p = get2::<ExternType, ExternType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            let p = get2::<ExternType, ExternType, _>(sub_ty, negated_ty);
            let inner = self
                .is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
                    env,
                    unsafe { &*p.first },
                    unsafe { &*p.second },
                    scope,
                );
            result = SubtypingResult::negate(&inner);
        } else if {
            let p = get2::<FunctionType, ExternType, _>(sub_ty, negated_ty);
            !p.first.is_null()
        } {
            result = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
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
        }

        result.with_super_component(Component::TypeField(TypeField::Negated));
        result
    }
}
