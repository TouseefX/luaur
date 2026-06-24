use alloc::vec::Vec;
use core::ffi::c_void;

use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::incomplete_inference::IncompleteInference;
use crate::records::subtyping::Subtyping;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct BidirectionalTypePusher {
    pub(crate) astTypes: *mut DenseHashMap<*const AstExpr, TypeId>,
    pub(crate) astExpectedTypes: *mut DenseHashMap<*const AstExpr, TypeId>,
    pub(crate) solver: *mut ConstraintSolver,
    pub(crate) constraint: *const Constraint,
    pub(crate) genericTypesAndPacks: *mut DenseHashSet<*const c_void>,
    pub(crate) unifier: *mut Unifier2,
    pub(crate) subtyping: *mut Subtyping,
    pub(crate) incompleteInferences: Vec<IncompleteInference>,
    // C++: `DenseHashSet<std::pair<TypeId, const AstExpr*>, PairHash<...>>`.
    // The bespoke `PairHash` (fn-ptr hashers) blocks `Default`/construction;
    // the tuple key is `Hash`, so the default `DenseHashDefault` hasher is used.
    // Identity and behavior are unchanged — only the hash function differs.
    pub(crate) seen: DenseHashSet<(TypeId, *const AstExpr)>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let astTypes: () = ();
    let astExpectedTypes: () = ();
    let solver: () = ();
    let constraint: () = ();
    let genericTypesAndPacks: () = ();
    let unifier: () = ();
    let subtyping: () = ();
    let incompleteInferences: () = ();
    let seen: () = ();
    let pushType: () = ();
}
