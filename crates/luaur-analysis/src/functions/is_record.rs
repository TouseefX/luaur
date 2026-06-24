use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_table::Item;
use luaur_ast::records::ast_expr_table::ItemKind;

pub fn is_record(item: &Item) -> bool {
    if item.kind == ItemKind::Record {
        return true;
    } else if item.kind == ItemKind::General {
        if item.key.is_null() {
            return false;
        }

        if luaur_ast::rtti::ast_node_is::<AstExprConstantString>(unsafe {
            &*(item.key as *mut luaur_ast::records::ast_node::AstNode)
        }) {
            return true;
        }

        false
    } else {
        false
    }
}
