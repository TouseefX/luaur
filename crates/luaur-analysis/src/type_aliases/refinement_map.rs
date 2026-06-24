//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/LValue.h:46:refinement_map`
//! Source: `Analysis/include/Luau/LValue.h` (LValue.h:46, hand-ported)

use crate::type_aliases::l_value::LValue;
use crate::type_aliases::type_id::TypeId;

// C++: using RefinementMap = std::unordered_map<LValue, TypeId, LValueHasher>;
pub type RefinementMap = std::collections::HashMap<LValue, TypeId>;
