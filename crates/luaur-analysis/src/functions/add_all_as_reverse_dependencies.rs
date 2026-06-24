//! @interface-stub
use crate::records::checkpoint::Checkpoint;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn add_all_as_reverse_dependencies(
    start: Checkpoint,
    end: Checkpoint,
    cg: &ConstraintGenerator,
    target: *mut Constraint,
) {
    LUAU_ASSERT!(luaur_common::FFlag::LuauConstraintGraph.get());

    for i in start.offset..end.offset {
        let constraint = cg.constraints[i];

        unsafe {
            (*cg.cgraph).add_dependency_of_constraint_constraint(&mut *target, &mut *constraint);
        }
    }
}
