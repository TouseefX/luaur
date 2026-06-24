use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;

pub fn string_part_of_interp_string(node: *const AstNode, position: Position) -> bool {
    if node.is_null() {
        return false;
    }

    let interp_string = unsafe { (*node).as_item::<AstExprInterpString>() };

    if interp_string.is_null() {
        return false;
    }

    let expressions = unsafe { &(*interp_string).expressions };

    for expression in unsafe { expressions.as_slice() } {
        let expression = *expression;
        if expression.is_null() {
            continue;
        }

        let expr_location = unsafe { &(*expression).base.location };
        if expr_location.contains(position) {
            return false;
        }
    }

    true
}
