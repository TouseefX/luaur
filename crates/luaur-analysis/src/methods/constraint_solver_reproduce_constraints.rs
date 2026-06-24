use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::reduce_pack_constraint::ReducePackConstraint;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::constraint_v::ConstraintV;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn reproduce_constraints(
        &mut self,
        scope: NonNull<Scope>,
        location: Location,
        subst: &Substitution,
    ) {
        for (_, &new_ty) in subst.new_types.iter() {
            if !unsafe { get_type_id::<TypeFunctionInstanceType>(new_ty) }.is_null() {
                self.push_constraint(
                    scope,
                    location,
                    ConstraintV::Reduce(ReduceConstraint { ty: new_ty }),
                );
            }
        }

        for (_, &new_pack) in subst.new_packs.iter() {
            if !unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(new_pack) }.is_null() {
                self.push_constraint(
                    scope,
                    location,
                    ConstraintV::ReducePack(ReducePackConstraint { tp: new_pack }),
                );
            }
        }
    }
}
