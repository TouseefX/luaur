use crate::functions::end_type_pack::end_type_pack_id;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn flatten(tp: TypePackId, log: &TxnLog) -> (alloc::vec::Vec<TypeId>, Option<TypePackId>) {
    let tp = log.follow_type_pack_id(tp);
    let mut flattened = alloc::vec::Vec::new();
    let mut it = TypePackIterator::type_pack_iterator();
    it.type_pack_iterator_type_pack_id_txn_log(tp, log as *const TxnLog);

    while it.operator_ne(&end_type_pack_id(tp)) {
        flattened.push(*it.operator_deref());
        it.operator_inc();
    }

    let tail = it.tail();
    (flattened, tail)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use flatten as flatten_type_pack_id_txn_log;
