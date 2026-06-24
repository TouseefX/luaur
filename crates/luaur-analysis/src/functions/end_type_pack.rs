use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn end_type_pack_id(_tp: TypePackId) -> TypePackIterator {
    TypePackIterator::type_pack_iterator()
}

pub fn end(tp: TypePackId) -> TypePackIterator {
    end_type_pack_id(tp)
}
