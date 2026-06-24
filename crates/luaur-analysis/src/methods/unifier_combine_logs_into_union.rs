use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::unifier::Unifier;

impl Unifier {
    pub fn unifier_combine_logs_into_union(
        &mut self,
        logs: Vec<TxnLog>,
        arena: *mut TypeArena,
    ) -> TxnLog {
        let mut result = TxnLog::new();
        for log in logs {
            result.concat_as_union(log, arena);
        }
        result
    }
}
