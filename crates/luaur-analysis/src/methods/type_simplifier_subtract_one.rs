use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::negation_type::NegationType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn subtract_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
        let builtin_types = unsafe { &*self.builtin_types };
        let target = unsafe { follow_type_id(target) };
        let discriminant = unsafe { follow_type_id(discriminant) };
        if let Some(nt) = unsafe { get_type_id::<NegationType>(discriminant).as_ref() } {
            return self.intersect_one(target, nt.ty);
        }
        match relate_type_id_type_id(target, discriminant) {
            Relation::Disjoint => Some(target),
            Relation::Subset | Relation::Coincident => Some(builtin_types.neverType),
            _ => None,
        }
    }
}
