use crate::enums::inhabited::Inhabited;
use crate::functions::intersect_one_with_intersection::intersect_one_with_intersection;
use crate::records::type_ids::TypeIds;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn intersect_from_parts(&mut self, mut parts: TypeIds) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };

        if parts.size() == 0 {
            return builtin_types.unknownType;
        }

        if parts.size() == 1 {
            return parts.front();
        }

        let mut source = TypeIds::type_ids();
        let mut dest = TypeIds::type_ids();

        source.reserve(parts.size());
        dest.reserve(parts.size());

        for i in 0..parts.size() {
            let part = unsafe { *parts.order.as_ptr().add(i) };

            if intersect_one_with_intersection(self, &mut source, &mut dest, part) == Inhabited::No
            {
                return builtin_types.neverType;
            }

            core::mem::swap(&mut source, &mut dest);
            dest.clear_without_realloc();
        }

        let mut ib =
            crate::records::intersection_builder::IntersectionBuilder::intersection_builder(
                self.arena as *mut _,
                builtin_types as *const _ as *mut _,
            );

        for &ty in &source.order {
            ib.add(ty);
        }

        ib.build()
    }
}
