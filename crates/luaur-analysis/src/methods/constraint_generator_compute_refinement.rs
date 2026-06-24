use crate::enums::table_state::TableState;
use crate::functions::contains_subscripted_definition::contains_subscripted_definition;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::conjunction_refinement::Conjunction;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::disjunction_refinement::Disjunction;
use crate::records::equivalence::Equivalence;
use crate::records::negation_refinement::Negation;
use crate::records::negation_type::NegationType;
use crate::records::property_type::Property;
use crate::records::proposition_refinement::Proposition;
use crate::records::refinement_partition::RefinementPartition;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::records::variadic::Variadic;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::refinement_context::RefinementContext;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::refinement_refinement::{Refinement, RefinementMember};
use alloc::collections::BTreeMap;
use core::mem::ManuallyDrop;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    // ConstraintGenerator::computeRefinement(const ScopePtr&, Location, RefinementId,
    //     RefinementContext*, bool sense, bool eq, std::vector<ConstraintV>*)
    // (ConstraintGenerator.cpp:565).
    pub fn compute_refinement(
        &mut self,
        scope: *mut Scope,
        location: Location,
        refinement: RefinementId,
        refis: *mut RefinementContext,
        sense: bool,
        eq: bool,
        constraints: *mut alloc::vec::Vec<ConstraintV>,
    ) {
        if refinement.is_null() {
            return;
        }

        let refinement_ref: &Refinement = unsafe { &*refinement };

        if let Some(variadic) = <Variadic as RefinementMember>::get_if(refinement_ref) {
            for refi in variadic.refinements.clone() {
                self.compute_refinement(scope, location, refi, refis, sense, eq, constraints);
            }
        } else if let Some(negation) = <Negation as RefinementMember>::get_if(refinement_ref) {
            self.compute_refinement(
                scope,
                location,
                negation.refinement,
                refis,
                !sense,
                eq,
                constraints,
            );
        } else if let Some(conjunction) = <Conjunction as RefinementMember>::get_if(refinement_ref)
        {
            let (lhs, rhs) = (conjunction.lhs, conjunction.rhs);
            let mut lhs_refis = RefinementContext::default();
            let mut rhs_refis = RefinementContext::default();

            let lhs_target: *mut RefinementContext = if sense { refis } else { &mut lhs_refis };
            self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints);
            let rhs_target: *mut RefinementContext = if sense { refis } else { &mut rhs_refis };
            self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints);

            if !sense {
                let sp =
                    ManuallyDrop::new(unsafe { alloc::sync::Arc::from_raw(scope as *const Scope) });
                self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints);
            }
        } else if let Some(disjunction) = <Disjunction as RefinementMember>::get_if(refinement_ref)
        {
            let (lhs, rhs) = (disjunction.lhs, disjunction.rhs);
            let mut lhs_refis = RefinementContext::default();
            let mut rhs_refis = RefinementContext::default();

            let lhs_target: *mut RefinementContext = if sense { &mut lhs_refis } else { refis };
            self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints);
            let rhs_target: *mut RefinementContext = if sense { &mut rhs_refis } else { refis };
            self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints);

            if sense {
                let sp =
                    ManuallyDrop::new(unsafe { alloc::sync::Arc::from_raw(scope as *const Scope) });
                self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints);
            }
        } else if let Some(equivalence) = <Equivalence as RefinementMember>::get_if(refinement_ref)
        {
            let (lhs, rhs) = (equivalence.lhs, equivalence.rhs);
            self.compute_refinement(scope, location, lhs, refis, sense, true, constraints);
            self.compute_refinement(scope, location, rhs, refis, sense, true, constraints);
        } else if let Some(proposition) = <Proposition as RefinementMember>::get_if(refinement_ref)
        {
            let mut discriminant_ty = proposition.discriminantTy;
            let prop_key = proposition.key;
            let implicit_from_call = proposition.implicitFromCall;

            // if we have a negative sense, then we need to negate the discriminant
            if !sense {
                let nt = unsafe { get_type_id::<NegationType>(follow_type_id(discriminant_ty)) };
                if !nt.is_null() {
                    discriminant_ty = unsafe { (*nt).ty };
                } else {
                    discriminant_ty = unsafe {
                        (*self.arena).add_type(NegationType {
                            ty: discriminant_ty,
                        })
                    };
                }
            }

            if eq {
                let sp =
                    ManuallyDrop::new(unsafe { alloc::sync::Arc::from_raw(scope as *const Scope) });
                let singleton_func = unsafe { &(*self.builtin_types).typeFunctions.singleton_func };
                discriminant_ty = self.create_type_function_instance(
                    singleton_func,
                    alloc::vec![discriminant_ty],
                    alloc::vec![],
                    &sp,
                    location,
                );
            }

            let mut key = prop_key;
            while !key.is_null() {
                let key_def = unsafe { (*key).def } as DefId;

                unsafe {
                    (*refis).insert(key_def, RefinementPartition::default());
                    (*refis)
                        .get_mut(&key_def)
                        .unwrap()
                        .discriminant_types
                        .push(discriminant_ty);
                }

                // Reached leaf node
                let prop_name = unsafe { (*key).propName.clone() };
                let prop_name = match prop_name {
                    Some(n) => n,
                    None => break,
                };

                let mut props: BTreeMap<Name, Property> = BTreeMap::new();
                props.insert(prop_name, Property::readonly(discriminant_ty));

                let next_discriminant_ty = unsafe {
                    let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                        &props,
                        None,
                        (*scope).level,
                        scope,
                        TableState::Sealed,
                    );
                    (*self.arena).add_type(tt)
                };

                discriminant_ty = next_discriminant_ty;

                key = unsafe { (*key).parent };
            }

            // When the top-level expression is `t[x]`, we want to refine it into `nil`, not `never`.
            let prop_def = unsafe { (*prop_key).def } as DefId;
            LUAU_ASSERT!(unsafe { (*refis).get(&prop_def) }.is_some());
            unsafe {
                (*refis).get_mut(&prop_def).unwrap().should_append_nil_type = (sense || !eq)
                    && contains_subscripted_definition(prop_def)
                    && !implicit_from_call;
            }
        }
    }
}
