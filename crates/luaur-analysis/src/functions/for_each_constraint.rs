use crate::records::checkpoint::Checkpoint;
use crate::records::constraint_generator::ConstraintGenerator;

#[allow(non_snake_case)]
pub fn for_each_constraint<F>(
    start: Checkpoint,
    end: Checkpoint,
    cg: &ConstraintGenerator,
    mut f: F,
) where
    F: FnMut(*mut crate::records::constraint::Constraint),
{
    for i in start.offset..end.offset {
        f(cg.constraints[i]);
    }
}
