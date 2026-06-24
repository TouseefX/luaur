//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyTables, L1829-2149)
use crate::enums::table_state::TableState;
use crate::enums::variance::Variance;
use crate::functions::get_mutable_txn_log::get_mutable_pending_type;
use crate::functions::is_optional::is_optional;
use crate::functions::is_prim::is_prim;
use crate::functions::maybe_string::maybe_string;
use crate::records::missing_properties::{Context as MissingPropertiesContext, MissingProperties};
use crate::records::primitive_type::Type as PrimType;
use crate::records::table_type::TableType;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::records::unifier::Unifier;
use crate::type_aliases::literal_properties::LiteralProperties;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use std::collections::HashMap;

impl Unifier {
    /// `void Unifier::tryUnifyTables(TypeId subTy, TypeId superTy, bool isIntersection, const LiteralProperties* literalProperties)`
    pub fn unifier_try_unify_tables(
        &mut self,
        mut sub_ty: TypeId,
        mut super_ty: TypeId,
        is_intersection: bool,
        literal_properties: *const LiteralProperties,
    ) {
        if is_prim(self.log.follow_type_id(sub_ty), PrimType::Table) {
            sub_ty = unsafe { (*self.builtin_types).emptyTableType };
        }

        if is_prim(self.log.follow_type_id(super_ty), PrimType::Table) {
            super_ty = unsafe { (*self.builtin_types).emptyTableType };
        }

        let active_sub_ty = sub_ty;
        let mut super_table = self.log.txn_log_get_mutable::<TableType, TypeId>(super_ty);
        let mut sub_table = self.log.txn_log_get_mutable::<TableType, TypeId>(sub_ty);

        if super_table.is_null() || sub_table.is_null() {
            self.ice_string("passed non-table types to unifyTables");
        }

        let mut missing_properties: Vec<String> = Vec::new();
        let mut extra_properties: Vec<String> = Vec::new();

        if luaur_common::FFlag::LuauInstantiateInSubtyping.get() {
            if self.variance == Variance::Covariant
                && unsafe { (*sub_table).state } == TableState::Generic
                && unsafe { (*super_table).state } != TableState::Generic
            {
                // The Instantiation machinery is translated elsewhere in this crate;
                // keep this branch structurally present but avoid inventing
                // construction APIs (mirrors Unifier::tryUnifyFunctions). The C++
                // failure path here reports UnificationTooComplex.
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
                );
            }
        }

        // Optimization: First test that the property sets are compatible without doing any recursive unification
        if unsafe { (*sub_table).indexer.is_none() && (*sub_table).state != TableState::Free } {
            for (prop_name, super_prop) in unsafe { (*super_table).props.iter() } {
                let sub_has = unsafe { (*sub_table).props.contains_key(prop_name) };

                if !sub_has
                    && unsafe { (*sub_table).state } == TableState::Unsealed
                    && !is_optional(super_prop.type_deprecated())
                {
                    missing_properties.push(prop_name.clone());
                }
            }

            if !missing_properties.is_empty() {
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::MissingProperties(MissingProperties {
                        super_type: super_ty,
                        sub_type: sub_ty,
                        properties: core::mem::take(&mut missing_properties),
                        context: MissingPropertiesContext::Missing,
                    }),
                );
                return;
            }
        }

        // And vice versa if we're invariant
        if self.variance == Variance::Invariant
            && unsafe { (*super_table).indexer.is_none() }
            && unsafe { (*super_table).state } != TableState::Unsealed
            && unsafe { (*super_table).state } != TableState::Free
        {
            for (prop_name, _sub_prop) in unsafe { (*sub_table).props.iter() } {
                if unsafe { !(*super_table).props.contains_key(prop_name) } {
                    extra_properties.push(prop_name.clone());
                }
            }

            if !extra_properties.is_empty() {
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::MissingProperties(MissingProperties {
                        super_type: super_ty,
                        sub_type: sub_ty,
                        properties: core::mem::take(&mut extra_properties),
                        context: MissingPropertiesContext::Extra,
                    }),
                );
                return;
            }
        }

        // Width subtyping: any property in the supertype must be in the subtype,
        // and the types must agree.
        let super_props: Vec<String> = unsafe { (*super_table).props.keys().cloned().collect() };
        for name in super_props {
            let prop_ty = match unsafe { (*super_table).props.get(&name) } {
                Some(p) => p.type_deprecated(),
                None => continue,
            };
            let sub_prop = unsafe { (*sub_table).props.get(&name).map(|p| p.clone()) };

            if let Some(sub_prop) = sub_prop {
                // TODO: read-only properties don't need invariance
                let old_variance = self.variance;
                if literal_properties.is_null()
                    || unsafe { (*literal_properties).find(&name).is_none() }
                {
                    self.variance = Variance::Invariant;
                }

                let mut inner_state = self.unifier_make_child_unifier();
                inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                    sub_prop.type_deprecated(),
                    prop_ty,
                    false,
                    false,
                    None,
                );

                let inner_errors = inner_state.errors.clone();
                self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                    &inner_errors,
                    &name,
                    super_ty,
                    sub_ty,
                );

                if inner_state.errors.is_empty() {
                    self.log.concat(inner_state.log);
                }
                self.failure |= inner_state.failure;
                self.variance = old_variance;
            } else if unsafe {
                (*sub_table)
                    .indexer
                    .as_ref()
                    .map_or(false, |ix| maybe_string(ix.index_type))
            } {
                // TODO: read-only indexers don't need invariance
                let old_variance = self.variance;
                if literal_properties.is_null()
                    || unsafe { (*literal_properties).find(&name).is_none() }
                {
                    self.variance = Variance::Invariant;
                }

                let index_result =
                    unsafe { (*sub_table).indexer.as_ref().unwrap().index_result_type };
                let mut inner_state = self.unifier_make_child_unifier();
                inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                    index_result,
                    prop_ty,
                    false,
                    false,
                    None,
                );

                let inner_errors = inner_state.errors.clone();
                self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                    &inner_errors,
                    &name,
                    super_ty,
                    sub_ty,
                );

                if inner_state.errors.is_empty() {
                    self.log.concat(inner_state.log);
                }
                self.failure |= inner_state.failure;
                self.variance = old_variance;
            } else if unsafe { (*sub_table).state } == TableState::Unsealed && is_optional(prop_ty)
            {
                // This is sound because unsealed table types are precise.
            } else if unsafe { (*sub_table).state } == TableState::Free {
                let prop_clone = unsafe { (*super_table).props.get(&name).unwrap().clone() };
                let pending_sub = self.log.queue_type_id(active_sub_ty);
                let ttv = unsafe { get_mutable_pending_type::<TableType>(pending_sub) };
                LUAU_ASSERT!(!ttv.is_null());
                unsafe { (*ttv).props.insert(name.clone(), prop_clone) };
                sub_table = ttv;
            } else {
                missing_properties.push(name.clone());
            }

            // Recursive unification can change the txn log, and invalidate the old
            // table. If we detect that this has happened, we start over.
            let super_ty_new = self.log.follow_type_id(super_ty);
            let sub_ty_new = self.log.follow_type_id(active_sub_ty);

            if (super_ty != super_ty_new || active_sub_ty != sub_ty_new) && self.errors.is_empty() {
                return self.try_unify_type_id_type_id_bool_bool_literal_properties(
                    sub_ty,
                    super_ty,
                    false,
                    is_intersection,
                    None,
                );
            }

            let new_super_table = self
                .log
                .txn_log_get_mutable::<TableType, TypeId>(super_ty_new);
            let new_sub_table = self
                .log
                .txn_log_get_mutable::<TableType, TypeId>(sub_ty_new);

            if super_table != new_super_table || sub_table != new_sub_table {
                if self.errors.is_empty() {
                    self.unifier_try_unify_tables(
                        sub_ty,
                        super_ty,
                        is_intersection,
                        core::ptr::null(),
                    );
                }
                return;
            }
        }

        let sub_props: Vec<String> = unsafe { (*sub_table).props.keys().cloned().collect() };
        for name in sub_props {
            let prop = match unsafe { (*sub_table).props.get(&name) } {
                Some(p) => p.clone(),
                None => continue,
            };

            if unsafe { (*super_table).props.contains_key(&name) } {
                // already unified above
            } else if unsafe {
                (*super_table)
                    .indexer
                    .as_ref()
                    .map_or(false, |ix| maybe_string(ix.index_type))
            } {
                let old_variance = self.variance;
                if literal_properties.is_null()
                    || unsafe { (*literal_properties).find(&name).is_none() }
                {
                    self.variance = Variance::Invariant;
                }

                let super_index_result =
                    unsafe { (*super_table).indexer.as_ref().unwrap().index_result_type };
                let mut inner_state = self.unifier_make_child_unifier();
                if luaur_common::FFlag::LuauFixIndexerSubtypingOrdering.get() {
                    inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                        prop.type_deprecated(),
                        super_index_result,
                        false,
                        false,
                        None,
                    );
                } else {
                    // Incredibly, the old solver depends on this bug somehow.
                    inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                        super_index_result,
                        prop.type_deprecated(),
                        false,
                        false,
                        None,
                    );
                }

                let inner_errors = inner_state.errors.clone();
                self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                    &inner_errors,
                    &name,
                    super_ty,
                    sub_ty,
                );

                if inner_state.errors.is_empty() {
                    self.log.concat(inner_state.log);
                }
                self.failure |= inner_state.failure;
                self.variance = old_variance;
            } else if unsafe { (*super_table).state } == TableState::Unsealed {
                let mut clone = prop.clone();
                let deep =
                    self.unifier_deeply_optional(clone.type_deprecated(), &mut HashMap::new());
                clone.set_type(deep);

                let pending_super = self.log.queue_type_id(super_ty);
                let pending_super_ttv =
                    unsafe { get_mutable_pending_type::<TableType>(pending_super) };
                unsafe { (*pending_super_ttv).props.insert(name.clone(), clone) };
                super_table = pending_super_ttv;
            } else if self.variance == Variance::Covariant {
                // nothing
            } else if unsafe { (*super_table).state } == TableState::Free {
                let pending_super = self.log.queue_type_id(super_ty);
                let pending_super_ttv =
                    unsafe { get_mutable_pending_type::<TableType>(pending_super) };
                unsafe {
                    (*pending_super_ttv)
                        .props
                        .insert(name.clone(), prop.clone())
                };
                super_table = pending_super_ttv;
            } else {
                extra_properties.push(name.clone());
            }

            let super_ty_new = self.log.follow_type_id(super_ty);
            let sub_ty_new = self.log.follow_type_id(active_sub_ty);

            if (super_ty != super_ty_new || active_sub_ty != sub_ty_new) && self.errors.is_empty() {
                return self.try_unify_type_id_type_id_bool_bool_literal_properties(
                    sub_ty,
                    super_ty,
                    false,
                    is_intersection,
                    None,
                );
            }

            let new_super_table = self
                .log
                .txn_log_get_mutable::<TableType, TypeId>(super_ty_new);
            let new_sub_table = self
                .log
                .txn_log_get_mutable::<TableType, TypeId>(sub_ty_new);

            if super_table != new_super_table || sub_table != new_sub_table {
                if self.errors.is_empty() {
                    self.unifier_try_unify_tables(
                        sub_ty,
                        super_ty,
                        is_intersection,
                        core::ptr::null(),
                    );
                }
                return;
            }
        }

        // Unify indexers
        let super_has_indexer = unsafe { (*super_table).indexer.is_some() };
        let sub_has_indexer = unsafe { (*sub_table).indexer.is_some() };

        if super_has_indexer && sub_has_indexer {
            let old_variance = self.variance;
            self.variance = Variance::Invariant;

            let sub_index_type = unsafe { (*sub_table).indexer.as_ref().unwrap().index_type };
            let super_index_type = unsafe { (*super_table).indexer.as_ref().unwrap().index_type };
            let sub_index_result =
                unsafe { (*sub_table).indexer.as_ref().unwrap().index_result_type };
            let super_index_result =
                unsafe { (*super_table).indexer.as_ref().unwrap().index_result_type };

            let mut inner_state = self.unifier_make_child_unifier();

            inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_index_type,
                super_index_type,
                false,
                false,
                None,
            );

            let reported = !inner_state.errors.is_empty();

            let inner_errors = inner_state.errors.clone();
            self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                &inner_errors,
                "[indexer key]",
                super_ty,
                sub_ty,
            );

            inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_index_result,
                super_index_result,
                false,
                false,
                None,
            );

            if !reported {
                let inner_errors = inner_state.errors.clone();
                self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
                    &inner_errors,
                    "[indexer value]",
                    super_ty,
                    sub_ty,
                );
            }

            if inner_state.errors.is_empty() {
                self.log.concat(inner_state.log);
            }
            self.failure |= inner_state.failure;
            self.variance = old_variance;
        } else if super_has_indexer {
            if unsafe { (*sub_table).state } == TableState::Unsealed
                || unsafe { (*sub_table).state } == TableState::Free
            {
                let indexer = unsafe { (*super_table).indexer.clone() };
                self.log.change_indexer(sub_ty, indexer);
            }
        } else if sub_has_indexer && self.variance == Variance::Invariant {
            // Symmetric if we are invariant
            if unsafe { (*super_table).state } == TableState::Unsealed
                || unsafe { (*super_table).state } == TableState::Free
            {
                let indexer = unsafe { (*sub_table).indexer.clone() };
                self.log.change_indexer(super_ty, indexer);
            }
        }

        // Changing the indexer can invalidate the table pointers.
        let super_ty_f = self.log.follow_type_id(super_ty);
        let sub_ty_f = self.log.follow_type_id(active_sub_ty);
        super_table = self
            .log
            .txn_log_get_mutable::<TableType, TypeId>(super_ty_f);
        sub_table = self.log.txn_log_get_mutable::<TableType, TypeId>(sub_ty_f);

        if super_table.is_null() || sub_table.is_null() {
            return;
        }

        if !missing_properties.is_empty() {
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::MissingProperties(MissingProperties {
                    super_type: super_ty,
                    sub_type: sub_ty,
                    properties: core::mem::take(&mut missing_properties),
                    context: MissingPropertiesContext::Missing,
                }),
            );
            return;
        }

        if !extra_properties.is_empty() {
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::MissingProperties(MissingProperties {
                    super_type: super_ty,
                    sub_type: sub_ty,
                    properties: core::mem::take(&mut extra_properties),
                    context: MissingPropertiesContext::Extra,
                }),
            );
            return;
        }

        // Types are commonly cyclic; unifying a property may change the table itself.
        if unsafe { (*super_table).bound_to.is_some() || (*sub_table).bound_to.is_some() } {
            return self.try_unify_type_id_type_id_bool_bool_literal_properties(
                sub_ty, super_ty, false, false, None,
            );
        }

        if unsafe { (*super_table).state } == TableState::Free {
            self.log.bind_table(super_ty, Some(sub_ty));
        } else if unsafe { (*sub_table).state } == TableState::Free {
            self.log.bind_table(sub_ty, Some(super_ty));
        }
    }
}
