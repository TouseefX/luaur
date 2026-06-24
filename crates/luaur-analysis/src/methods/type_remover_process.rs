use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::r#type::Type;
use crate::records::type_ids::TypeIds;
use crate::records::type_remover::TypeRemover;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeRemover {
    /// C++ `void TypeRemover::process(TypeId item)` (Generalization.cpp:669-719).
    pub fn process(&mut self, item: TypeId) {
        let item = unsafe { follow_type_id(item) };

        // If we've already visited this item, or it's outside our arena, then
        // do not try to mutate it.
        if self.seen.contains(&item)
            || unsafe { (*item).owning_arena } != self.arena
            || unsafe { (*item).persistent }
        {
            return;
        }
        self.seen.insert(item);

        let ut = unsafe { get_mutable_type_id::<UnionType>(item) };
        if !ut.is_null() {
            let options: alloc::vec::Vec<TypeId> = unsafe { (*ut).options.clone() };
            let mut new_options = TypeIds::type_ids();
            for option in options {
                self.process(option);
                let option = unsafe { follow_type_id(option) };
                if option != self.needle
                    && unsafe { get_type_id::<NeverType>(option) }.is_null()
                    && option != item
                {
                    new_options.insert_type_id(option);
                }
            }
            let old_size = unsafe { (*ut).options.len() };
            if old_size != new_options.size() {
                if new_options.empty() {
                    emplace_bound_type(item, self.builtin_neverType());
                } else if new_options.size() == 1 {
                    let first = new_options.front();
                    emplace_bound_type(item, first);
                } else {
                    let taken = new_options.take();
                    let new_ty = unsafe { (*self.arena).add_type(UnionType { options: taken }) };
                    emplace_bound_type(item, new_ty);
                }
            }
            return;
        }

        let it = unsafe { get_mutable_type_id::<IntersectionType>(item) };
        if !it.is_null() {
            let parts: alloc::vec::Vec<TypeId> = unsafe { (*it).parts.clone() };
            let mut new_parts = TypeIds::type_ids();
            for part in parts {
                self.process(part);
                let part = unsafe { follow_type_id(part) };
                if part != self.needle
                    && unsafe { get_type_id::<UnknownType>(part) }.is_null()
                    && part != item
                {
                    new_parts.insert_type_id(part);
                }
            }
            let old_size = unsafe { (*it).parts.len() };
            if old_size != new_parts.size() {
                if new_parts.empty() {
                    emplace_bound_type(item, self.builtin_unknownType());
                } else if new_parts.size() == 1 {
                    let first = new_parts.front();
                    emplace_bound_type(item, first);
                } else {
                    let taken = new_parts.take();
                    let new_ty =
                        unsafe { (*self.arena).add_type(IntersectionType { parts: taken }) };
                    emplace_bound_type(item, new_ty);
                }
            }
        }
    }

    #[inline]
    fn builtin_neverType(&self) -> TypeId {
        unsafe { (*self.builtin_types).neverType }
    }

    #[inline]
    fn builtin_unknownType(&self) -> TypeId {
        unsafe { (*self.builtin_types).unknownType }
    }
}

/// C++ `emplaceType<BoundType>(asMutable(item), boundTo)`: replace the type's
/// variant in place with a `BoundType` pointing at `bound_to`.
fn emplace_bound_type(item: TypeId, bound_to: TypeId) {
    unsafe {
        let m = item as *mut Type;
        (*m).ty = TypeVariant::Bound(bound_to);
    }
}
