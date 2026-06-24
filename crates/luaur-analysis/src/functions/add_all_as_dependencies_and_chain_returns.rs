//! @interface-stub
use crate::records::checkpoint::Checkpoint;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintVMember;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn add_all_as_dependencies_and_chain_returns(
    start: Checkpoint,
    end: Checkpoint,
    cg: &ConstraintGenerator,
    target: *mut Constraint,
) {
    LUAU_ASSERT!(luaur_common::FFlag::LuauConstraintGraph.get());

    let mut previous: *mut Constraint = core::ptr::null_mut();

    for i in start.offset..end.offset {
        let constraint = cg.constraints[i];

        unsafe {
            (*cg.cgraph).add_dependency_of_constraint_constraint(&mut *constraint, &mut *target);

            if let Some(psc) = PackSubtypeConstraint::get_if(&(*constraint).c) {
                if psc.returns {
                    if !previous.is_null() {
                        (*cg.cgraph).add_dependency_of_constraint_constraint(
                            &mut *previous,
                            &mut *constraint,
                        );
                    }

                    previous = constraint;
                }
            }
        }
    }
}
