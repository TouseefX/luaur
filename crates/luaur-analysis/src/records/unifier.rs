//! Source: `Analysis/include/Luau/Unifier.h` (hand-ported; fields only)

use crate::enums::variance::Variance;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

#[derive(Debug)]
pub struct Unifier {
    pub types: *mut TypeArena,
    pub builtin_types: *mut BuiltinTypes,
    pub normalizer: *mut Normalizer,
    pub scope: *mut Scope,
    pub log: TxnLog,
    pub failure: bool,
    pub errors: ErrorVec,
    pub location: Location,
    pub variance: Variance,
    pub normalize: bool,
    pub check_inhabited: bool,
    pub ctx: CountMismatchContext,
    pub shared_state: *mut UnifierSharedState, // UnifierSharedState&
    pub blocked_types: Vec<TypeId>,
    pub blocked_type_packs: Vec<TypePackId>,
    pub first_pack_error_pos: Option<i32>,
}
