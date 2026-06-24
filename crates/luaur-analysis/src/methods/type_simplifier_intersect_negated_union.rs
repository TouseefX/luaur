use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::negation_type::NegationType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn intersect_negated_union(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };
        let left_negation = unsafe { get_type_id::<NegationType>(left).as_ref().unwrap() };
        let negated_ty = unsafe { follow_type_id(left_negation.ty) };
        let negated_union = unsafe { get_type_id::<UnionType>(negated_ty).as_ref().unwrap() };
        let mut changed = false;
        let mut new_parts = crate::records::type_ids::TypeIds::type_ids();
        for &part in &negated_union.options {
            match relate_type_id_type_id(part, right) {
                Relation::Disjoint => new_parts.insert_type_id(right),
                Relation::Coincident | Relation::Superset => return builtin_types.neverType,
                _ => {
                    let simplified =
                        self.intersect_type_with_negation(self.mk_negation(part), right);
                    changed |= simplified != right;
                    if unsafe {
                        !get_type_id::<crate::records::never_type::NeverType>(simplified).is_null()
                    } {
                        changed = true;
                    } else {
                        new_parts.insert_type_id(simplified);
                    }
                }
            }
        }
        if !changed {
            return right;
        }
        self.intersect_from_parts(new_parts)
    }
}
