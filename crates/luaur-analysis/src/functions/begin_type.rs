use crate::records::union_type::UnionType;
use crate::type_aliases::union_type_iterator::UnionTypeIterator;

pub fn begin_union_type(utv: &UnionType) -> UnionTypeIterator {
    UnionTypeIterator::type_iterator_type(utv as *const UnionType)
}
