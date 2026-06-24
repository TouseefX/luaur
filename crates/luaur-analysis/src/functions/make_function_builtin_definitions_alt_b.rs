use crate::functions::make_function_builtin_definitions_alt_d::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

pub fn make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool(
    arena: &mut TypeArena,
    self_type: Option<TypeId>,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
    param_types: Vec<TypeId>,
    ret_types: Vec<TypeId>,
    checked: bool,
) -> TypeId {
    make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool(arena, self_type, generics, generic_packs, param_types, Vec::new(), ret_types, checked)
}
