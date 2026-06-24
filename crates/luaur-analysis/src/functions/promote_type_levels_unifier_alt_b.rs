use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;

#[allow(non_snake_case)]
pub fn promote_type_levels_txn_log_type_arena_type_level_type_pack_id(
    log: &mut TxnLog,
    type_arena: &TypeArena,
    min_level: TypeLevel,
    tp: TypePackId,
) {
    // Type levels of types from other modules are already global, so we don't need to promote anything inside
    if unsafe { (*tp).owningArena != type_arena as *const TypeArena as *mut TypeArena } {
        return;
    }

    let mut ptl = PromoteTypeLevels::new(log, type_arena, min_level);
    // C++ `ptl.traverse(tp)` (Unifier.cpp:140) — drive the real GenericTypeVisitor
    // traversal so the per-node visits recurse and `log.changeLevel(...)` fires.
    ptl.traverse_type_pack_id(tp);
}
