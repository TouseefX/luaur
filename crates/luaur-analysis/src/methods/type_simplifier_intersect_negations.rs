use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::negation_type::NegationType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn intersect_negations(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let left_negation = unsafe { get_type_id::<NegationType>(left).as_ref().unwrap() };
        if unsafe { !get_type_id::<UnionType>(follow_type_id(left_negation.ty)).is_null() } {
            return self.intersect_negated_union(left, right);
        }
        let right_negation = unsafe { get_type_id::<NegationType>(right).as_ref().unwrap() };
        if unsafe { !get_type_id::<UnionType>(follow_type_id(right_negation.ty)).is_null() } {
            return self.intersect_negated_union(right, left);
        }
        match relate_type_id_type_id(left_negation.ty, right_negation.ty) {
            Relation::Coincident | Relation::Superset => left,
            Relation::Subset => right,
            _ => unsafe {
                (*self.arena.cast_mut()).add_type(
                    crate::records::intersection_type::IntersectionType {
                        parts: alloc::vec![left, right],
                    },
                )
            },
        }
    }
}
