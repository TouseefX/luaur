use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn begin(tp: TypePackId) -> TypePackIterator {
    let mut it = TypePackIterator::type_pack_iterator();
    it.type_pack_iterator_type_pack_id(tp);
    it
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use begin as begin_type_pack_id;
