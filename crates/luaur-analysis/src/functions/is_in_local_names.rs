use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

use luaur_ast::records::position::Position;

pub fn is_in_local_names(ancestry: &alloc::vec::Vec<*mut AstNode>, position: Position) -> bool {
    let mut iter = ancestry.len();
    while iter > 0 {
        iter -= 1;
        let node = ancestry[iter];
        if node.is_null() {
            continue;
        }

        if unsafe { ast_node_is::<AstStatLocal>(&*(node as *mut AstNode)) } {
            let stat_local = unsafe { ast_node_as::<AstStatLocal>(node as *mut AstNode) };
            let vars = unsafe { &(*stat_local).vars };
            for var in vars {
                let var_location = unsafe { (*(*var)).location };
                if var_location.containsClosed(position) {
                    return true;
                }
            }
        } else if unsafe { ast_node_is::<AstExprFunction>(&*(node as *mut AstNode)) } {
            let func_expr = unsafe { ast_node_as::<AstExprFunction>(node as *mut AstNode) };
            let arg_location = unsafe { (*func_expr).arg_location };
            if let Some(arg_loc) = arg_location {
                if arg_loc.contains(position) {
                    return true;
                }
            }
        } else if unsafe { ast_node_is::<AstStatLocalFunction>(&*(node as *mut AstNode)) } {
            let local_func = unsafe { ast_node_as::<AstStatLocalFunction>(node as *mut AstNode) };
            let name_location = unsafe { (*(*local_func).name).location };
            if name_location.containsClosed(position) {
                return true;
            }
        } else if unsafe { ast_node_is::<AstStatBlock>(&*(node as *mut AstNode)) } {
            let block = unsafe { ast_node_as::<AstStatBlock>(node as *mut AstNode) };
            let body = unsafe { &(*block).body };
            if body.len() > 0 {
                return false;
            }
        } else if unsafe { (*node).as_stat_const() }.is_null() {
            // If it's not a stat, continue searching
        } else {
            return false;
        }
    }

    false
}
