use crate::records::checkpoint::Checkpoint;
use crate::records::constraint_generator::ConstraintGenerator;

pub fn checkpoint(cg: *const ConstraintGenerator) -> Checkpoint {
    // SAFETY: The caller guarantees `cg` is a valid pointer to a `ConstraintGenerator`.
    // The `constraints` field is a `Vec<ConstraintPtr>`, and `size()` returns its length.
    let offset = unsafe { (*cg).constraints.len() };
    Checkpoint { offset }
}
