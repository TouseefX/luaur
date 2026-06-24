use crate::functions::add_intersection::add_intersection;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn intersect_unions(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let left_union_ptr = unsafe { get_type_id::<UnionType>(left) };
        LUAU_ASSERT!(!left_union_ptr.is_null());
        let left_union = unsafe { &*left_union_ptr };

        let right_union_ptr = unsafe { get_type_id::<UnionType>(right) };
        LUAU_ASSERT!(!right_union_ptr.is_null());
        let right_union = unsafe { &*right_union_ptr };

        let mut _new_parts: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null_mut());

        // Combinatorial blowup moment!!

        // combination size
        let option_size = left_union.options.len() * right_union.options.len();
        let max_size = luaur_common::DFInt::LuauSimplificationComplexityLimit.get() as usize;

        if option_size > max_size {
            return crate::functions::add_intersection::add_intersection(
                self.arena as *mut crate::records::type_arena::TypeArena,
                self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
                &[left, right],
            );
        }

        let mut ub = UnionBuilder::union_builder(
            self.arena as *mut crate::records::type_arena::TypeArena,
            self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
        );

        for &left_part in &left_union.options {
            for &right_part in &right_union.options {
                let simplified = self.intersect(left_part, right_part);
                ub.add(simplified);

                // Initial combination size check could not predict nested union iteration
                if ub.size() > max_size {
                    return add_intersection(
                        self.arena as *mut crate::records::type_arena::TypeArena,
                        self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
                        &[left, right],
                    );
                }
            }
        }

        ub.build()
    }
}
