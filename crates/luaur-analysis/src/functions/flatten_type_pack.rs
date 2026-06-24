use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn flatten_type_pack_id(tp: TypePackId) -> (alloc::vec::Vec<TypeId>, Option<TypePackId>) {
    let mut res = alloc::vec::Vec::new();

    let mut iter = begin(tp);
    let end_iter = end(tp);

    while iter.operator_ne(&end_iter) {
        res.push(*iter.operator_deref());
        iter.operator_inc();
    }

    (res, iter.tail())
}
