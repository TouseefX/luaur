use crate::functions::add_intersection::add_intersection;
use crate::functions::begin_type::begin_union_type;
use crate::functions::end_type::end_union_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::DFInt;

impl TypeSimplifier {
    pub fn intersect_union_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let left_union_ptr = unsafe { get_type_id::<UnionType>(left) };
        LUAU_ASSERT!(!left_union_ptr.is_null());
        let left_union = unsafe { &*left_union_ptr };

        let mut changed = false;
        let max_size = DFInt::LuauSimplificationComplexityLimit.get() as usize;

        if left_union.options.len() > max_size {
            return add_intersection(
                self.arena as *mut crate::records::type_arena::TypeArena,
                self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
                &[left, right],
            );
        }

        let mut ub = UnionBuilder::union_builder(
            self.arena as *mut crate::records::type_arena::TypeArena,
            self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
        );
        ub.reserve(left_union.options.len());

        // `for (TypeId part : leftUnion)` uses the cycle-protecting
        // `UnionTypeIterator`, which follows and flattens nested unions and
        // skips ones it has already visited — without it, a cyclic union here
        // recurses forever.
        let mut iter = begin_union_type(left_union);
        let end = end_union_type(left_union);
        while iter.operator_ne(&end) {
            let part = iter.operator_deref();

            let simplified = self.intersect(right, part);
            changed |= simplified != part;

            let never_ptr = unsafe { get_type_id::<NeverType>(simplified) };
            if !never_ptr.is_null() {
                changed = true;
                iter.operator_inc();
                continue;
            }

            ub.add(simplified);

            // Initial combination size check could not predict nested union iteration
            if ub.size() > max_size {
                return add_intersection(
                    self.arena as *mut crate::records::type_arena::TypeArena,
                    self.builtin_types as *mut crate::records::builtin_types::BuiltinTypes,
                    &[left, right],
                );
            }

            iter.operator_inc();
        }

        if !changed {
            return left;
        }

        ub.build()
    }
}
