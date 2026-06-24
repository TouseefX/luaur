use crate::enums::relation::Relation;
use crate::functions::add_intersection::add_intersection;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_type_variable::is_type_variable;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::DFInt;

impl TypeSimplifier {
    pub fn intersect_intersection_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };
        let left_intersection = unsafe { get_type_id::<IntersectionType>(left).as_ref().unwrap() };

        if left_intersection.parts.len() > DFInt::LuauSimplificationComplexityLimit.get() as usize {
            return add_intersection(
                self.arena as *mut crate::records::type_arena::TypeArena,
                self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
                &[left, right],
            );
        }

        let mut changed = false;
        let mut new_parts = crate::records::type_ids::TypeIds::type_ids();
        for &part in &left_intersection.parts {
            match relate_type_id_type_id(part, right) {
                Relation::Disjoint => return builtin_types.neverType,
                Relation::Coincident => new_parts.insert_type_id(part),
                Relation::Subset => new_parts.insert_type_id(part),
                Relation::Superset => {
                    new_parts.insert_type_id(right);
                    changed = true;
                }
                Relation::Intersects => {
                    new_parts.insert_type_id(part);
                    new_parts.insert_type_id(right);
                    changed = true;
                }
            }
        }

        // It is sometimes the case that an intersection operation will result in
        // clipping a free type from the result.
        //
        // eg (number & 'a) & string --> never
        //
        // We want to only report the free types that are part of the result.
        for &part in &new_parts.order {
            if is_type_variable(part) {
                self.blocked_types.insert(part);
            }
        }

        if !changed {
            return left;
        }

        self.intersect_from_parts(new_parts)
    }
}
