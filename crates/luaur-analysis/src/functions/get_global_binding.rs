//! Node: `cxx:Function:Luau.Analysis:Analysis/src/BuiltinDefinitions.cpp:308:get_global_binding`
//! Source: `Analysis/src/BuiltinDefinitions.cpp:308-313` (hand-ported)

use crate::functions::try_get_global_binding::try_get_global_binding;
use crate::records::global_types::GlobalTypes;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `TypeId getGlobalBinding(GlobalTypes& globals, const std::string& name)`.
pub fn get_global_binding(globals: &mut GlobalTypes, name: &str) -> TypeId {
    let binding = try_get_global_binding(globals, name);
    LUAU_ASSERT!(binding.is_some());
    binding.unwrap().type_id
}
