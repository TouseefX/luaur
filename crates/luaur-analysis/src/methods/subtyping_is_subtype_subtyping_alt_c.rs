//! Faithful port of the 5-arg pack overload
//! `Subtyping::isSubtype(subTp, superTp, scope, bindableGenerics, bindableGenericPacks)`
//! (Analysis/src/Subtyping.cpp:641-675).
use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_bounds::GenericBounds;
use crate::records::generic_type::GenericType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Subtyping {
    /// C++:
    /// ```cpp
    /// SubtypingResult Subtyping::isSubtype(
    ///     TypePackId subTp, TypePackId superTp, NotNull<Scope> scope,
    ///     const std::vector<TypeId>& bindableGenerics,
    ///     const std::vector<TypePackId>& bindableGenericPacks)
    /// {
    ///     SubtypingEnvironment env;
    ///     for (TypeId g : bindableGenerics)
    ///         env.mappedGenerics[follow(g)] = {SubtypingEnvironment::GenericBounds{}};
    ///     env.mappedGenericPacks.pushFrame(bindableGenericPacks);
    ///     SubtypingResult result = isCovariantWith(env, subTp, superTp, scope);
    ///     for (TypeId bg : bindableGenerics) {
    ///         bg = follow(bg);
    ///         LUAU_ASSERT(env.mappedGenerics.contains(bg));
    ///         if (const std::vector<SubtypingEnvironment::GenericBounds>* bounds = env.mappedGenerics.find(bg)) {
    ///             LUAU_ASSERT(bounds->size() == 1);
    ///             if (bounds->empty()) continue;
    ///             if (const GenericType* gen = get<GenericType>(bg))
    ///                 result.andAlso(checkGenericBounds(bounds->back(), env, scope, gen->name));
    ///         }
    ///     }
    ///     return result;
    /// }
    /// ```
    pub fn is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id_vector_type_pack_id(
        &mut self,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        scope: *mut Scope,
        bindable_generics: &Vec<TypeId>,
        bindable_generic_packs: &Vec<TypePackId>,
    ) -> SubtypingResult {
        let mut env = SubtypingEnvironment {
            parent: core::ptr::null_mut(),
            mapped_generics: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null(),
            ),
            mapped_generic_packs:
                crate::records::mapped_generic_environment::MappedGenericEnvironment {
                    frames: alloc::vec::Vec::new(),
                    current_scope_index: None,
                },
            substitutions: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null(),
            ),
            seen_set_cache: luaur_common::records::dense_hash_map::DenseHashMap::new((
                core::ptr::null(),
                core::ptr::null(),
            )),
            iteration_count: 0,
        };

        for &g in bindable_generics.iter() {
            *env.mapped_generics
                .get_or_insert(unsafe { follow_type_id(g) }) =
                alloc::vec![GenericBounds::default()];
        }

        env.mapped_generic_packs.push_frame(bindable_generic_packs);

        let mut result = self
            .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
                &mut env, sub_tp, super_tp, scope,
            );

        for &bg in bindable_generics.iter() {
            let bg = unsafe { follow_type_id(bg) };

            LUAU_ASSERT!(env.mapped_generics.contains(&bg));

            // Clone the bounds out so the immutable borrow of `env` is released
            // before the `&mut env` call to `checkGenericBounds`.
            let last_bounds = match env.mapped_generics.find(&bg) {
                Some(bounds) => {
                    // Bounds should have exactly one entry
                    LUAU_ASSERT!(bounds.len() == 1);
                    if bounds.is_empty() {
                        continue;
                    }
                    bounds.last().unwrap().clone()
                }
                None => continue,
            };

            if let Some(gen) = unsafe { get_type_id::<GenericType>(bg).as_ref() } {
                let generic_name = gen.name.clone();
                let bounds_result = self.subtyping_check_generic_bounds(
                    &last_bounds,
                    &mut env,
                    scope,
                    &generic_name,
                );
                result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);
            }
        }

        result
    }
}
