use crate::functions::make_function_builtin_definitions_alt_d::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;

pub fn make_function(
    arena: &mut TypeArena,
    self_type: Option<TypeId>,
    param_types: Vec<TypeId>,
    param_names: Vec<String>,
    ret_types: Vec<TypeId>,
    checked: bool,
) -> TypeId {
    make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool(arena, self_type, Vec::new(), Vec::new(), param_types, param_names, ret_types, checked)
}
