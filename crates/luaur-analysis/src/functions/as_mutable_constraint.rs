use crate::records::constraint::Constraint;

pub fn as_mutable_constraint(c: &Constraint) -> *mut Constraint {
    c as *const Constraint as *mut Constraint
}
