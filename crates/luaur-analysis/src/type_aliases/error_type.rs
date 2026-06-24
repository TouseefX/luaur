//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Type.h:813:error_type`
//! Source: `Analysis/include/Luau/Type.h:813` — `using ErrorType = Unifiable::Error<TypeId>` (hand-ported)

use crate::type_aliases::type_id::TypeId;

pub type ErrorType = crate::records::unifiable::Error<TypeId>;
