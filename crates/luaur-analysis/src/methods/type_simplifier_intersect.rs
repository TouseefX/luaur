use crate::functions::add_intersection::add_intersection;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_type_variable::is_type_variable;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::any_type::AnyType;
use crate::records::error_type::ErrorType;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn intersect(&mut self, mut left: TypeId, mut right: TypeId) -> TypeId {
        let mut rl = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        rl.recursion_limiter_recursion_limiter(
            "TypeSimplifier::intersect",
            &mut self.recursion_depth,
            15,
        );

        left = self.simplify_type_id(left);
        right = self.simplify_type_id(right);

        if left == right {
            return left;
        }

        let bt = unsafe { &*self.builtin_types };
        if !unsafe { get_type_id::<AnyType>(left) }.is_null()
            && !unsafe { get_type_id::<ErrorType>(right) }.is_null()
        {
            return right;
        }
        if !unsafe { get_type_id::<AnyType>(right) }.is_null()
            && !unsafe { get_type_id::<ErrorType>(left) }.is_null()
        {
            return left;
        }
        if !unsafe { get_type_id::<UnknownType>(left) }.is_null()
            && unsafe { get_type_id::<ErrorType>(right) }.is_null()
        {
            return right;
        }
        if !unsafe { get_type_id::<UnknownType>(right) }.is_null()
            && unsafe { get_type_id::<ErrorType>(left) }.is_null()
        {
            return left;
        }
        if !unsafe { get_type_id::<AnyType>(left) }.is_null()
            && !unsafe { get_type_id::<UnionType>(right) }.is_null()
        {
            return self.union_(bt.errorType, right);
        }
        if !unsafe { get_type_id::<UnionType>(left) }.is_null()
            && !unsafe { get_type_id::<AnyType>(right) }.is_null()
        {
            return self.union_(bt.errorType, left);
        }
        if !unsafe { get_type_id::<AnyType>(left) }.is_null() {
            return unsafe { &mut *(self.arena as *mut crate::records::type_arena::TypeArena) }
                .add_type(UnionType {
                    options: alloc::vec![right, bt.errorType],
                });
        }
        if !unsafe { get_type_id::<AnyType>(right) }.is_null() {
            return unsafe { &mut *(self.arena as *mut crate::records::type_arena::TypeArena) }
                .add_type(UnionType {
                    options: alloc::vec![left, bt.errorType],
                });
        }
        if !unsafe { get_type_id::<UnknownType>(left) }.is_null() {
            return right;
        }
        if !unsafe { get_type_id::<UnknownType>(right) }.is_null() {
            return left;
        }
        if !unsafe { get_type_id::<NeverType>(left) }.is_null() {
            return left;
        }
        if !unsafe { get_type_id::<NeverType>(right) }.is_null() {
            return right;
        }

        if let Some(lf) = unsafe { get_type_id::<FreeType>(left).as_ref() } {
            if relate_type_id_type_id(lf.upper_bound, right)
                == crate::enums::relation::Relation::Subset
                || relate_type_id_type_id(lf.upper_bound, right)
                    == crate::enums::relation::Relation::Coincident
            {
                return left;
            }
        } else if let Some(rf) = unsafe { get_type_id::<FreeType>(right).as_ref() } {
            if relate_type_id_type_id(left, rf.upper_bound)
                == crate::enums::relation::Relation::Superset
                || relate_type_id_type_id(left, rf.upper_bound)
                    == crate::enums::relation::Relation::Coincident
            {
                return right;
            }
        }

        if is_type_variable(left) {
            self.blocked_types.insert(left);
            return add_intersection(
                self.arena as *mut _,
                self.builtin_types as *mut _,
                &[left, right],
            );
        }
        if is_type_variable(right) {
            self.blocked_types.insert(right);
            return add_intersection(
                self.arena as *mut _,
                self.builtin_types as *mut _,
                &[left, right],
            );
        }

        if !unsafe { get_type_id::<UnionType>(left) }.is_null() {
            return if !unsafe { get_type_id::<UnionType>(right) }.is_null() {
                self.intersect_unions(left, right)
            } else {
                self.intersect_union_with_type(left, right)
            };
        } else if !unsafe { get_type_id::<UnionType>(right) }.is_null() {
            return self.intersect_union_with_type(right, left);
        }

        if !unsafe { get_type_id::<IntersectionType>(left) }.is_null() {
            return self.intersect_intersection_with_type(left, right);
        } else if !unsafe { get_type_id::<IntersectionType>(right) }.is_null() {
            return self.intersect_intersection_with_type(right, left);
        }

        if !unsafe { get_type_id::<NegationType>(left) }.is_null() {
            return if !unsafe { get_type_id::<NegationType>(right) }.is_null() {
                self.intersect_negations(left, right)
            } else {
                self.intersect_type_with_negation(left, right)
            };
        } else if !unsafe { get_type_id::<NegationType>(right) }.is_null() {
            return self.intersect_type_with_negation(right, left);
        }

        if let Some(res) = self.basic_intersect(left, right) {
            res
        } else {
            unsafe { &mut *(self.arena as *mut crate::records::type_arena::TypeArena) }.add_type(
                IntersectionType {
                    parts: alloc::vec![left, right],
                },
            )
        }
    }
}
