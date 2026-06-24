use crate::enums::relation::Relation;
use crate::functions::relate_simplify::relate;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::simplifier_seen_set::SimplifierSeenSet;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn relate_type_id_type_id(left: TypeId, right: TypeId) -> Relation {
    let mut seen = DenseHashMap::new((core::ptr::null(), core::ptr::null()));
    relate(left, right, &mut seen)
}
