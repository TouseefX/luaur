use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;
use luaur_common::records::variant::Variant2;

#[allow(non_camel_case_types)]
pub type IncompatibilityReason = Variant2<SubtypingReasonings, ErrorVec>;
