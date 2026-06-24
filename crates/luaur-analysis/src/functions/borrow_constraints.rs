use crate::records::constraint::Constraint;
use crate::type_aliases::constraint_ptr::ConstraintPtr;

pub fn borrow_constraints(
    constraints: &alloc::vec::Vec<ConstraintPtr>,
) -> alloc::vec::Vec<*mut Constraint> {
    let mut result: alloc::vec::Vec<*mut Constraint> =
        alloc::vec::Vec::with_capacity(constraints.len());

    for &c in constraints.iter() {
        result.push(c);
    }

    result
}
