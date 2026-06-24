use crate::functions::find_type_element_at_autocomplete_core::find_type_element_at_ast_type_list_type_pack_id_position;
use crate::functions::find_type_element_at_autocomplete_core_alt_c::find_type_element_at_ast_type_type_id_position;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;
use luaur_ast::records::position::Position;

pub fn find_type_element_at_ast_type_pack_type_pack_id_position(
    ast_type_pack: *mut AstTypePack,
    tp: TypePackId,
    position: Position,
) -> Option<TypeId> {
    unsafe {
        if !ast_type_pack.is_null() {
            let node = ast_type_pack as *mut luaur_ast::records::ast_node::AstNode;
            if luaur_ast::rtti::ast_node_is::<AstTypePackExplicit>(&*node) {
                let explicit = ast_type_pack as *mut AstTypePackExplicit;
                let type_list = (*explicit).type_list;
                return find_type_element_at_ast_type_list_type_pack_id_position(
                    &type_list, tp, position,
                );
            } else if luaur_ast::rtti::ast_node_is::<AstTypePackVariadic>(&*node) {
                let variadic = ast_type_pack as *mut AstTypePackVariadic;
                let loc = (*ast_type_pack).base.location;
                if loc.containsClosed(position) {
                    let (_, tail) = flatten_type_pack_id(tp);

                    if let Some(tail_id) = tail {
                        let follow_tp = follow_type_pack_id(tail_id);
                        let vtp = get_type_pack_id::<VariadicTypePack>(follow_tp);
                        if !vtp.is_null() {
                            let vtp_ref = &*vtp;
                            let variadic_type = (*variadic).variadic_type;
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
    }
    None
}
