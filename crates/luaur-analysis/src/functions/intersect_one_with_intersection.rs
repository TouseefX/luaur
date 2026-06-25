use crate::enums::inhabited::Inhabited;
use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_ids::TypeIds;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;

pub fn intersect_one_with_intersection(
    simplifier: &mut TypeSimplifier,
    source: &mut TypeIds,
    dest: &mut TypeIds,
    candidate: TypeId,
) -> Inhabited {
    // `get<T>` requires a followed type (it asserts the argument is not a BoundType).
    // The C++ pipeline guarantees the candidate is followed at this point (TypeIds
    // entries are followed on insert); a recursed sub-part of an IntersectionType is
    // not necessarily followed, so we follow here to preserve that invariant.
    let candidate = unsafe { follow_type_id(candidate) };

    if dest.count(candidate) > 0 {
        return Inhabited::Yes;
    }

    if let Some(itv) = unsafe { get_type_id::<IntersectionType>(candidate).as_ref() } {
        for &sub_part in &itv.parts {
            if intersect_one_with_intersection(simplifier, source, dest, sub_part) == Inhabited::No
            {
                return Inhabited::No;
            }
        }

        return Inhabited::Yes;
    }

    if source.empty() {
        dest.insert_type_id(candidate);
        return Inhabited::Yes;
    }
    for i in 0..source.size() {
        let ty = unsafe { *source.order.as_ptr().add(i) };
        match relate_type_id_type_id(candidate, ty) {
            Relation::Disjoint => return Inhabited::No,
            Relation::Subset => dest.insert_type_id(candidate),
            Relation::Coincident | Relation::Superset => dest.insert_type_id(ty),
            Relation::Intersects => {
                if let Some(simplified) = simplifier.basic_intersect(candidate, ty) {
                    dest.insert_type_id(simplified);
                } else {
                    dest.insert_type_id(candidate);
                    dest.insert_type_id(ty);
                }
            }
        }
    }
    Inhabited::Yes
}
