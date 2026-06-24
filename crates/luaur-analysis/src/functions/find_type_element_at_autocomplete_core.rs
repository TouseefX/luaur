use crate::functions::find_type_element_at_autocomplete_core_alt_c::find_type_element_at_ast_type_type_id_position;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;
use luaur_ast::records::position::Position;

pub fn find_type_element_at_ast_type_list_type_pack_id_position(
    ast_type_list: &AstTypeList,
    tp: TypePackId,
    position: Position,
) -> Option<TypeId> {
    let types = unsafe {
        std::slice::from_raw_parts(ast_type_list.types.data, ast_type_list.types.size as usize)
    };

    for i in 0..types.len() {
        let type_ = types[i];

        let location = unsafe { (*type_).base.location };
        if location.containsClosed(position) {
            let (head, _) = flatten_type_pack_id(tp);

            if i < head.len() {
                return find_type_element_at_ast_type_type_id_position(type_, head[i], position);
            }
        }
    }

    if !ast_type_list.tail_type.is_null() {
        let arg_tp = unsafe { &*ast_type_list.tail_type };

        let variadic = unsafe {
            luaur_ast::rtti::ast_node_as::<AstTypePackVariadic>(
                &arg_tp.base as *const luaur_ast::records::ast_node::AstNode
                    as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        if !variadic.is_null() {
            let location = unsafe { (*variadic).base.base.location };
            if location.containsClosed(position) {
                let (_, tail) = flatten_type_pack_id(tp);

                if let Some(tail_id) = tail {
                    let follow_tp = unsafe { follow_type_pack_id(tail_id) };
                    let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(follow_tp) };
                    if !vtp.is_null() {
                        let vtp_ref = unsafe { &*vtp };
                        let variadic_type = unsafe { (*variadic).variadic_type };
                        return find_type_element_at_ast_type_type_id_position(
                            variadic_type,
                            vtp_ref.ty,
                            position,
                        );
                    }
                }
            }
        }
    }

    None
}
