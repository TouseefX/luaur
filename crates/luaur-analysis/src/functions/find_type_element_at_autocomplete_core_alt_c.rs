use crate::functions::find_type_element_at_autocomplete_core::find_type_element_at_ast_type_list_type_pack_id_position;
use crate::functions::find_type_element_at_autocomplete_core_alt_b::find_type_element_at_ast_type_pack_type_pack_id_position;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::position::Position;

pub fn find_type_element_at_ast_type_type_id_position(
    ast_type: *mut AstType,
    ty: TypeId,
    position: Position,
) -> Option<TypeId> {
    let ty = unsafe { follow_type_id(ty) };

    unsafe {
        if (*ast_type).base.class_index == luaur_ast::rtti::ast_rtti_index("AstTypeReference") {
            return Some(ty);
        }

        if (*ast_type).base.class_index == luaur_ast::rtti::ast_rtti_index("AstTypeError") {
            return Some(ty);
        }

        let type_function = luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            ast_type as *mut luaur_ast::records::ast_node::AstNode,
        );
        if !type_function.is_null() {
            let ftv = get_type_id::<FunctionType>(ty);
            if ftv.is_null() {
                return None;
            }

            let arg_types = &(*type_function).arg_types;
            let arg_types_tp = (*ftv).arg_types;

            if let Some(element) = find_type_element_at_ast_type_list_type_pack_id_position(
                arg_types,
                arg_types_tp,
                position,
            ) {
                return Some(element);
            }

            let return_types = (*type_function).return_types;
            let ret_types_tp = (*ftv).ret_types;

            if let Some(element) = find_type_element_at_ast_type_pack_type_pack_id_position(
                return_types,
                ret_types_tp,
                position,
            ) {
                return Some(element);
            }
        }
    }

    None
}
