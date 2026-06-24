use crate::records::function_argument::FunctionArgument;
use crate::records::function_type::FunctionType;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

pub fn make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_string_initializer_list_type_id_bool(
    arena: &mut TypeArena,
    self_type: Option<TypeId>,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
    param_types: Vec<TypeId>,
    param_names: Vec<String>,
    ret_types: Vec<TypeId>,
    checked: bool,
) -> TypeId {
    let mut params = Vec::new();
    if let Some(st) = self_type {
        params.push(st);
    }
    params.extend(param_types.iter().cloned());

    let param_pack = arena.add_type_pack_initializer_list_type_id(&params);
    let ret_pack = arena.add_type_pack_initializer_list_type_id(&ret_types);

    let mut ftv = FunctionType::function_type_new(param_pack, ret_pack, None, self_type.is_some());
    ftv.generics = generics;
    ftv.generic_packs = generic_packs;
    ftv.is_checked_function = checked;

    if self_type.is_some() {
        ftv.arg_names.push(Some(FunctionArgument {
            name: "self".into(),
            location: Location::default(),
        }));
    }

    if !param_names.is_empty() {
        for p in param_names {
            ftv.arg_names.push(Some(FunctionArgument {
                name: p.into(),
                location: Location::default(),
            }));
        }
    } else if self_type.is_some() {
        for _ in 0..param_types.len() {
            ftv.arg_names.push(None);
        }
    }

    arena.add_type(ftv)
}
