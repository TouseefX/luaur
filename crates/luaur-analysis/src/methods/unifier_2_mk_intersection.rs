use crate::functions::follow_type::follow_type_id;
use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;

impl Unifier2 {
    pub fn mk_intersection(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let left = unsafe { follow_type_id(left) };
        let right = unsafe { follow_type_id(right) };
        let builtin_types = self.builtin_types.as_ptr();
        let arena = self.arena.as_ptr();
        simplify_intersection(builtin_types, arena, left, right).result
    }
}
