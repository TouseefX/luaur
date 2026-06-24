use crate::records::bidirectional_type_pusher::BidirectionalTypePusher;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::push_type_result::PushTypeResult;
use crate::records::subtyping::Subtyping;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn push_type_into(
    ast_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
    ast_expected_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
    solver: NonNull<ConstraintSolver>,
    constraint: NonNull<Constraint>,
    generic_types_and_packs: NonNull<DenseHashSet<*const c_void>>,
    unifier: NonNull<Unifier2>,
    subtyping: NonNull<Subtyping>,
    expected_type: TypeId,
    expr: *const AstExpr,
) -> PushTypeResult {
    let mut btp = BidirectionalTypePusher::bidirectional_type_pusher(
        ast_types,
        ast_expected_types,
        solver,
        constraint,
        generic_types_and_packs,
        unifier,
        subtyping,
    );

    btp.push_type(expected_type, expr);

    PushTypeResult {
        incomplete_types: btp.incompleteInferences,
    }
}
