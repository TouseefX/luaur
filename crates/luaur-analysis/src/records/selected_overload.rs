use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SelectedOverload {
    pub overload: Option<TypeId>,
    pub assumed_constraints: Vec<ConstraintV>,
    pub should_retry: bool,
}
