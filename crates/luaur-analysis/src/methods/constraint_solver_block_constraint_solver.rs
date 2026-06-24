//! Source: `Analysis/include/Luau/ConstraintSolver.h:330-337` (hand-ported)
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;

impl ConstraintSolver {
    /// C++ `template<typename T> bool block(const T& targets, NotNull<const Constraint> constraint)`
    /// iterates `targets` (a `TypeId` range) and forwards each to `block(TypeId, ...)`.
    /// In the Rust port the callers are monomorphized onto
    /// `block_type_id_not_null_constraint` / `block_type_pack_id_not_null_constraint`
    /// (and direct loops over the container), so this unbounded template generic has
    /// no call site and no iterable bound to port faithfully.
    pub fn block_t_not_null_constraint<T>(
        &mut self,
        _targets: &T,
        _constraint: *const Constraint,
    ) -> bool {
        unreachable!(
            "C++ ConstraintSolver::block<T> template generic; Rust callers use the monomorphized block_type_id/block_type_pack_id overloads — no call site"
        )
    }
}
