use crate::records::promote_type_levels::PromoteTypeLevels;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl PromoteTypeLevels {
    pub fn promote_type_levels_promote_type_levels(
        log: &mut TxnLog,
        type_arena: &TypeArena,
        min_level: TypeLevel,
    ) -> Self {
        let mut visitor = PromoteTypeLevels {
            base: TypeOnceVisitor::new("PromoteTypeLevels".to_string(), false),
            log: log as *mut TxnLog,
            type_arena: type_arena as *const TypeArena,
            min_level,
        };
        visitor
    }
}
