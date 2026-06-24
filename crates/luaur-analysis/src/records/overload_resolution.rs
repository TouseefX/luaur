use crate::records::selected_overload::SelectedOverload;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::incompatibility_reason::IncompatibilityReason;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct OverloadResolution {
    pub ok: Vec<TypeId>,
    pub non_functions: Vec<TypeId>,
    pub potential_overloads: Vec<(TypeId, Vec<ConstraintV>)>,
    pub incompatible_overloads: Vec<(TypeId, IncompatibilityReason)>,
    pub arity_mismatches: Vec<TypeId>,
    pub metamethods: DenseHashSet<TypeId>,
}
