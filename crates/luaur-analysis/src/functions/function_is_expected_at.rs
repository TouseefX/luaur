use crate::functions::find_expected_type_at::find_expected_type_at;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::module::Module;
use crate::records::union_type::UnionType;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;

pub fn function_is_expected_at(
    module: &Module,
    node: *mut AstNode,
    position: Position,
) -> Option<bool> {
    let type_at_position = find_expected_type_at(module, node, position)?;
    let expected_type = unsafe { follow_type_id(type_at_position) };

    unsafe {
        if !get_type_id::<FunctionType>(expected_type).is_null() {
            return Some(true);
        }

        if let Some(itv) = get_type_id::<IntersectionType>(expected_type).as_ref() {
            for part in &itv.parts {
                if get_type_id::<FunctionType>(follow_type_id(*part)).is_null() {
                    return Some(false);
                }
            }
            return Some(true);
        }

        if let Some(utv) = get_type_id::<UnionType>(expected_type).as_ref() {
            return Some(
                crate::functions::return_first_nonnull_option_of_type::return_first_nonnull_option_of_type::<FunctionType>(utv)
                    .is_some()
            );
        }

        Some(false)
    }
}
