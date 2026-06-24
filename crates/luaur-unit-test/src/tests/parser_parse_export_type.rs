#[cfg(test)]
#[test]
fn parser_parse_export_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_assign::AstStatAssign;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "export()\n\
         export = 5\n\
         export, export = export\n\
         export type A = number\n\
         type A = number",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block_ptr = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(
            stat as *mut _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!block_ptr.is_null());
    let block = unsafe { &*block_ptr };
    assert_eq!(5, block.body.size);

    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatExpr>(
                *block.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
            )
        }
        .is_null()
            == false
    );
    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatAssign>(
                *block.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
            )
        }
        .is_null()
            == false
    );
    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatAssign>(
                *block.body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode
            )
        }
        .is_null()
            == false
    );
    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
                *block.body.data.add(3) as *mut luaur_ast::records::ast_node::AstNode
            )
        }
        .is_null()
            == false
    );
    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
                *block.body.data.add(4) as *mut luaur_ast::records::ast_node::AstNode
            )
        }
        .is_null()
            == false
    );
}
