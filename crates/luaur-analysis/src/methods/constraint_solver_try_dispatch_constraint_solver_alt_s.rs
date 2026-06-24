use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::emplace_type::emplace_type;
use crate::functions::follow_type::follow_type_id;
use crate::records::bound_type::BoundType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::find_all_union_members::FindAllUnionMembers;
use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::simplify_constraint::SimplifyConstraint;
use crate::records::union_type::UnionType;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_simplify_constraint_not_null_constraint_bool(
        &mut self,
        c: &SimplifyConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let target = unsafe { follow_type_id(c.ty) };

        unsafe {
            if (*target).persistent
                || (*target).owning_arena != self.arena
                || crate::functions::get_type_alt_j::get_type_id::<UnionType>(target).is_null()
            {
                return true;
            }
        }

        let mut finder = FindAllUnionMembers::new();
        finder.traverse_type_id(target);

        if !finder.blocked_tys.empty() && !force {
            for ty in &finder.blocked_tys.order {
                self.block_type_id_not_null_constraint(*ty, constraint);
            }
            return false;
        }

        let mut result = unsafe { (*self.builtin_types).neverType };
        for ty in &finder.recorded_tys.order {
            let ty_followed = unsafe { follow_type_id(*ty) };
            if ty_followed == target {
                continue;
            }
            result = self.simplify_union(
                unsafe { (*constraint).scope },
                unsafe { (*constraint).location },
                result,
                ty_followed,
            );
        }

        if force {
            for ty in &finder.blocked_tys.order {
                let ty_followed = unsafe { follow_type_id(*ty) };
                if ty_followed == target {
                    continue;
                }
                result = self.simplify_union(
                    unsafe { (*constraint).scope },
                    unsafe { (*constraint).location },
                    result,
                    ty_followed,
                );
            }
        }

        let mutable_target = unsafe { as_mutable_type_id(target) };
        let mut result_arg = result;
        crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type(
            unsafe { &mut *mutable_target },
            &mut result_arg,
        );

        if FFlag::LuauConstraintGraph.get() {
            unsafe { (*self.cgraph).shift_references_type_id(target, result) };
        }

        true
    }
}
