use alloc::vec::Vec;

use luaur_common::records::dense_hash_set::DenseHashSet;

use crate::records::bidirectional_type_pusher::BidirectionalTypePusher;

impl BidirectionalTypePusher {
    /// `BidirectionalTypePusher::BidirectionalTypePusher(...)`
    /// (TableLiteralInference.cpp:96-113).
    pub fn bidirectional_type_pusher(
        ast_types: core::ptr::NonNull<
            luaur_common::records::dense_hash_map::DenseHashMap<
                *const luaur_ast::records::ast_expr::AstExpr,
                crate::type_aliases::type_id::TypeId,
            >,
        >,
        ast_expected_types: core::ptr::NonNull<
            luaur_common::records::dense_hash_map::DenseHashMap<
                *const luaur_ast::records::ast_expr::AstExpr,
                crate::type_aliases::type_id::TypeId,
            >,
        >,
        solver: core::ptr::NonNull<crate::records::constraint_solver::ConstraintSolver>,
        constraint: core::ptr::NonNull<crate::records::constraint::Constraint>,
        generic_types_and_packs: core::ptr::NonNull<
            luaur_common::records::dense_hash_set::DenseHashSet<*const core::ffi::c_void>,
        >,
        unifier: core::ptr::NonNull<crate::records::unifier_2::Unifier2>,
        subtyping: core::ptr::NonNull<crate::records::subtyping::Subtyping>,
    ) -> Self {
        BidirectionalTypePusher {
            astTypes: ast_types.as_ptr(),
            astExpectedTypes: ast_expected_types.as_ptr(),
            solver: solver.as_ptr(),
            constraint: constraint.as_ptr(),
            genericTypesAndPacks: generic_types_and_packs.as_ptr(),
            unifier: unifier.as_ptr(),
            subtyping: subtyping.as_ptr(),
            incompleteInferences: Vec::new(),
            // C++: `seen{{nullptr, nullptr}}` — empty-key sentinel is the null pair.
            seen: DenseHashSet::new((core::ptr::null(), core::ptr::null())),
        }
    }
}
