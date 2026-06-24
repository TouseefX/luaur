use crate::records::constraint_list::ConstraintList;
use crate::records::hash_blocked_constraint_id::HashBlockedConstraintId;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type ConstraintMap =
    DenseHashMap<ConstraintVertex, *mut ConstraintList, HashBlockedConstraintId>;
