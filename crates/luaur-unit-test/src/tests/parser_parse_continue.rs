#[cfg(test)]
#[test]
fn parser_parse_continue() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "while true do\n\
         continue()\n\
         continue = 5\n\
         continue, continue = continue\n\
         continue\n\
         end",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_block::AstStatBlock>(
            stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!block.is_null());
    assert_eq!(1, unsafe { (*block).body.size });

    let wb = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_while::AstStatWhile>(
            *(*block).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!wb.is_null());

    let wblock = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_block::AstStatBlock>(
            (*wb).body as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!wblock.is_null());
    assert_eq!(4, unsafe { (*wblock).body.size });

    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_expr::AstStatExpr>(
            *(*wblock).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_assign::AstStatAssign>(
            *(*wblock).body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_assign::AstStatAssign>(
            *(*wblock).body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_continue::AstStatContinue>(
            *(*wblock).body.data.add(3) as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
}
