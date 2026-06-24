use crate::records::constraint_step_snapshot::ConstraintStepSnapshot;
use crate::records::generalize_step_snapshot::GeneralizeStepSnapshot;
use luaur_common::records::variant::Variant2;

pub type StepSnapshot = Variant2<ConstraintStepSnapshot, GeneralizeStepSnapshot>;
