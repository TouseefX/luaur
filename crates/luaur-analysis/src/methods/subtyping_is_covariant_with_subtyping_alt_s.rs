//! Faithful port of the function/function `isCovariantWith` overload
//! `Subtyping::isCovariantWith(env, const FunctionType* subFunction, const FunctionType* superFunction, scope)`
//! (Analysis/src/Subtyping.cpp:2378-2486).
use crate::enums::pack_field::PackField;
use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::enums::subtyping_variance::SubtypingVariance;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::merge_reasonings::k_empty_reasoning;
use crate::records::function_type::FunctionType;
use crate::records::generic_bounds::GenericBounds;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_count_mismatch::GenericTypeCountMismatch;
use crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_error::TypeError;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::component::Component;
use crate::type_aliases::path::Path;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Subtyping {
    /// C++ `isContravariantWith(env, subTp, superTp, scope)` instantiated for the
    /// `TypePackId` overload set: `isCovariantWith(env, superTp, subTp, scope)`
    /// (note the swap) followed by the contravariant reasoning transform. We
    /// inline it here rather than routing through the generic `isContravariantWith`
    /// helper, because that helper's `IntoCovOperand` dispatch models only the
    /// `TypeId` / `TableIndexer` instantiations; the pack overload is selected by
    /// C++ overload resolution and must call the pack `isCovariantWith` directly.
    fn is_contravariant_with_packs(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = self
            .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
                env, super_tp, sub_tp, scope,
            );

        if result.reasoning.empty() {
            result.reasoning.insert(SubtypingReasoning {
                sub_path: Path::default(),
                super_path: Path::default(),
                variance: SubtypingVariance::Contravariant,
                is_property_modifier_violation: false,
            });
        } else {
            let mut updated = SubtypingReasonings::new(k_empty_reasoning());
            for r in result.reasoning.iter() {
                let mut r = r.clone();
                core::mem::swap(&mut r.sub_path, &mut r.super_path);
                if r.variance == SubtypingVariance::Covariant {
                    r.variance = SubtypingVariance::Contravariant;
                } else if r.variance == SubtypingVariance::Contravariant {
                    r.variance = SubtypingVariance::Covariant;
                }
                updated.insert(r);
            }
            result.reasoning = updated;
        }

        result
    }

    pub fn is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_function: &FunctionType,
        super_function: &FunctionType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult::default();

        if !sub_function.generics.is_empty() {
            for &g in sub_function.generics.iter() {
                let g = unsafe { follow_type_id(g) };
                if unsafe { !get_type_id::<GenericType>(g).is_null() } {
                    if let Some(bounds) = env.mapped_generics.find_mut(&g) {
                        // g may shadow an existing generic, so push a fresh set of bounds
                        bounds.push(GenericBounds::default());
                    } else {
                        *env.mapped_generics.get_or_insert(g) =
                            alloc::vec![GenericBounds::default()];
                    }
                }
            }
        }

        if !sub_function.generic_packs.is_empty() {
            let mut packs: alloc::vec::Vec<TypePackId> =
                alloc::vec::Vec::with_capacity(sub_function.generic_packs.len());

            for &g in sub_function.generic_packs.iter() {
                let g = unsafe { crate::functions::follow_type_pack::follow_type_pack_id(g) };
                if unsafe {
                    !crate::functions::get_type_pack::get_type_pack_id::<
                        crate::records::generic_type_pack::GenericTypePack,
                    >(g)
                    .is_null()
                } {
                    packs.push(g);
                }
            }

            env.mapped_generic_packs.push_frame(&packs);
        }

        {
            let mut arg_result = self.is_contravariant_with_packs(
                env,
                sub_function.arg_types,
                super_function.arg_types,
                scope,
            );
            arg_result.with_both_component(Component::PackField(PackField::Arguments));
            result.or_else(arg_result);

            // If subtyping failed in the argument packs, we should check if there's a hidden variadic tail and try ignoring it.
            // This might cause subtyping correctly because the sub type here may not have a hidden variadic tail or equivalent.
            if !result.is_subtype {
                let (arguments, tail) = flatten_type_pack_id(super_function.arg_types);

                let hidden_variadic = match tail {
                    Some(t) => {
                        let variadic = unsafe {
                            crate::functions::get_type_pack::get_type_pack_id::<VariadicTypePack>(t)
                        };
                        if let Some(v) = unsafe { variadic.as_ref() } {
                            v.hidden
                        } else {
                            false
                        }
                    }
                    None => false,
                };

                if hidden_variadic {
                    let truncated = unsafe {
                        (*self.arena)
                            .add_type_pack_vector_type_id_optional_type_pack_id(arguments, None)
                    };
                    let mut retry = self.is_contravariant_with_packs(
                        env,
                        sub_function.arg_types,
                        truncated,
                        scope,
                    );
                    retry.with_both_component(Component::PackField(PackField::Arguments));
                    result.or_else(retry);
                }
            }
        }

        let mut ret_result = self
            .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
                env,
                sub_function.ret_types,
                super_function.ret_types,
                scope,
            );
        ret_result.with_both_component(Component::PackField(PackField::Returns));
        result.and_also(ret_result, SubtypingSuppressionPolicy::Any);

        if unsafe {
            (*sub_function.arg_types).type_pack_var_operator_eq(&*super_function.arg_types)
                && (*sub_function.ret_types).type_pack_var_operator_eq(&*super_function.ret_types)
        } {
            // It's fine to upcast a function with generics to a function without.
            // Intuitively: a generic function should always be a subtype of its instantiations.
            if super_function.generics.len() != sub_function.generics.len()
                && !super_function.generics.is_empty()
            {
                result.and_also(
                    SubtypingResult {
                        is_subtype: false,
                        ..Default::default()
                    },
                    SubtypingSuppressionPolicy::Any,
                );
                result.with_error(TypeError {
                    location: unsafe { (*scope).location.clone() },
                    module_name: alloc::string::String::new(),
                    data: TypeErrorData::GenericTypeCountMismatch(GenericTypeCountMismatch {
                        sub_ty_generic_count: super_function.generics.len(),
                        super_ty_generic_count: sub_function.generics.len(),
                    }),
                });
            }

            if super_function.generic_packs.len() != sub_function.generic_packs.len()
                && !super_function.generic_packs.is_empty()
            {
                result.and_also(
                    SubtypingResult {
                        is_subtype: false,
                        ..Default::default()
                    },
                    SubtypingSuppressionPolicy::Any,
                );
                result.with_error(TypeError {
                    location: unsafe { (*scope).location.clone() },
                    module_name: alloc::string::String::new(),
                    data: TypeErrorData::GenericTypePackCountMismatch(
                        GenericTypePackCountMismatch {
                            subTyGenericPackCount: super_function.generic_packs.len(),
                            superTyGenericPackCount: sub_function.generic_packs.len(),
                        },
                    ),
                });
            }
        }

        if !sub_function.generics.is_empty() {
            for &g in sub_function.generics.iter() {
                let g = unsafe { follow_type_id(g) };
                if let Some(gen) = unsafe { get_type_id::<GenericType>(g).as_ref() } {
                    let generic_name = gen.name.clone();

                    let last_bounds = {
                        let bounds = env.mapped_generics.find(&g);
                        LUAU_ASSERT!(bounds.is_some() && !bounds.unwrap().is_empty());
                        bounds.unwrap().last().unwrap().clone()
                    };

                    let bounds_result = self.subtyping_check_generic_bounds(
                        &last_bounds,
                        env,
                        scope,
                        &generic_name,
                    );
                    result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);

                    env.mapped_generics.find_mut(&g).unwrap().pop();
                }
            }
        }

        if !sub_function.generic_packs.is_empty() {
            env.mapped_generic_packs.pop_frame();
            // This result isn't cacheable, because we may need it to populate the generic pack mapping environment again later
            result.is_cacheable = false;
        }

        result
    }
}
