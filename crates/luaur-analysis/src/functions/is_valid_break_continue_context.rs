use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_is;

pub fn is_valid_break_continue_context(
    ancestry: &alloc::vec::Vec<*mut AstNode>,
    position: Position,
) -> bool {
    let mut iter = ancestry.len();
    while iter > 0 {
        iter -= 1;
        let node = ancestry[iter];

        if unsafe { ast_node_is::<AstStatFunction>(&*(node as *mut AstNode)) }
            || unsafe { ast_node_is::<AstStatLocalFunction>(&*(node as *mut AstNode)) }
            || unsafe { ast_node_is::<AstExprFunction>(&*(node as *mut AstNode)) }
            || unsafe { ast_node_is::<AstStatTypeFunction>(&*(node as *mut AstNode)) }
            || unsafe { ast_node_is::<AstTypeFunction>(&*(node as *mut AstNode)) }
        {
            return false;
        }

        if let Some(stat_while) = unsafe {
            ast_node_is::<AstStatWhile>(&*(node as *mut AstNode)).then(|| node as *mut AstStatWhile)
        } {
            let body_location = unsafe { (*(*stat_while).body).base.base.location };
            if body_location.contains(position) {
                return true;
            }
        }

        if let Some(stat_for) = unsafe {
            ast_node_is::<AstStatFor>(&*(node as *mut AstNode)).then(|| node as *mut AstStatFor)
        } {
            let body_location = unsafe { (*(*stat_for).body).base.base.location };
            if body_location.contains(position) {
                return true;
            }
        }

        if let Some(stat_for_in) = unsafe {
            ast_node_is::<AstStatForIn>(&*(node as *mut AstNode)).then(|| node as *mut AstStatForIn)
        } {
            let body_location = unsafe { (*(*stat_for_in).body).base.base.location };
            if body_location.contains(position) {
                return true;
            }
        }

        if let Some(stat_repeat) = unsafe {
            ast_node_is::<AstStatRepeat>(&*(node as *mut AstNode))
                .then(|| node as *mut AstStatRepeat)
        } {
            let body_location = unsafe { (*(*stat_repeat).body).base.base.location };
            if body_location.contains(position) {
                return true;
            }
        }
    }

    false
}
