use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::to_string_options::ToStringOptions;
use crate::records::type_ids::TypeIds;
use crate::records::unblocked_types::UnblockedTypes;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use crate::type_aliases::type_pack_ids::TypePackIds;
use core::ptr::NonNull;
use luaur_common::FFlag;

impl ConstraintGraph {
    pub fn unblock_constraint(&mut self, c: NonNull<Constraint>) -> UnblockedTypes {
        let mut result = UnblockedTypes {
            types: TypeIds::type_ids(),
            packs: TypePackIds::new(core::ptr::null_mut()),
        };

        // The reverse dependencies of this constraint should contain all of the types
        // and type packs that this constraint may mutate, either as a free type or
        // as a blocked type.
        let c_ptr = c.as_ptr() as *const Constraint;
        let reverse_deps = self.find_reverse_dependency_list(ConstraintVertex::V2(c_ptr));
        let reverse_deps_ref = unsafe { reverse_deps.as_ref() };

        for rdep in reverse_deps_ref.order.iter() {
            // The C++ `for (auto rdep : *reverseDeps)` iterates only present entries.
            if !reverse_deps_ref.contains(rdep.clone()) {
                continue;
            }

            if let Some(ty) = rdep.get_if_0() {
                let ty = *ty;
                result.types.insert_type_id(ty);
                let deps = self.find_dependency_list(ConstraintVertex::V0(ty));
                let deps_mut = unsafe { &mut *deps.as_ptr() };
                deps_mut.remove(ConstraintVertex::V2(c_ptr));
            } else if let Some(tp) = rdep.get_if_1() {
                let tp = *tp;
                let _ = result.packs.insert(tp);
                let deps = self.find_dependency_list(ConstraintVertex::V1(tp));
                let deps_mut = unsafe { &mut *deps.as_ptr() };
                deps_mut.remove(ConstraintVertex::V2(c_ptr));
            } else if let Some(dep_cons) = rdep.get_if_2() {
                let dep_cons = *dep_cons;
                let deps = self.find_dependency_list(ConstraintVertex::V2(dep_cons));
                let deps_mut = unsafe { &mut *deps.as_ptr() };
                deps_mut.remove(ConstraintVertex::V2(c_ptr));
                if FFlag::DebugLuauLogSolver.get() {
                    let mut opts = ToStringOptions::default();
                    opts.exhaustive = true;
                    std::print!(
                        "Unblocking count={}\t{}\n",
                        deps_mut.size() as i32,
                        to_string_constraint_to_string_options(unsafe { &*dep_cons }, &mut opts)
                    );
                }
            } else {
                luaur_common::macros::luau_assert::LUAU_ASSERT!(false);
            }
        }

        /*
         * This whole song and dance is to repair the constraint graph after we
         * dispatch a constraint.
         *
         * We are assuming that, after a constraint has been dispatched, some
         * number of mutations have been made to the type graph. Importantly: if a
         * type has been mutated, then it was previously a reverse dependency of
         * [c]. If that is the case, then we can walk the reverse deps of [c] and
         * try to find bound types, shift their references over to their bounds,
         * and "repair" the dependency graph without having to track every single
         * [bind] call.
         */

        for ty in result.types.order.iter() {
            self.repair_type_references_type_id(*ty);
        }

        let packs: alloc::vec::Vec<_> = result.packs.iter().copied().collect();
        for type_pack in packs {
            self.repair_type_references_type_pack_id(type_pack);
        }

        result
    }
}
