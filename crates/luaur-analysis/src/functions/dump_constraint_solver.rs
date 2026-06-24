//! `static void dump(ConstraintSolver* cs, ToStringOptions& opts)`
//! (`Analysis/src/ConstraintSolver.cpp:4315-4335`, hand-ported faithfully).

use core::ptr::NonNull;

use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::to_string_options::ToStringOptions;
use luaur_common::FFlag;

pub fn dump(cs: *mut ConstraintSolver, opts: &mut ToStringOptions) {
    let cs_ref = unsafe { &mut *cs };

    if FFlag::LuauConstraintGraph.get() {
        let unsolved: alloc::vec::Vec<NonNull<Constraint>> = cs_ref
            .unsolved_constraints
            .iter()
            .map(|c| NonNull::new(*c as *mut Constraint).unwrap())
            .collect();
        unsafe { (*cs_ref.cgraph).dump_with(&unsolved, opts) };
    } else {
        for c in cs_ref.unsolved_constraints.iter() {
            let c = *c;
            let block_count = cs_ref
                .deprecated_blocked_constraints
                .get(&c)
                .map(|v| *v as i32)
                .unwrap_or(0);
            std::print!(
                "\t{}\t{}\n",
                block_count,
                to_string_constraint_to_string_options(unsafe { &*c }, opts)
            );
            for dep in unsafe { &(*c).deprecated_dependencies } {
                let dep_const = *dep as *const Constraint;
                if cs_ref
                    .unsolved_constraints
                    .iter()
                    .any(|uc| *uc == dep_const)
                {
                    std::print!(
                        "\t\t|\t{}\n",
                        to_string_constraint_to_string_options(unsafe { &*dep_const }, opts)
                    );
                }
            }
        }
    }
}
