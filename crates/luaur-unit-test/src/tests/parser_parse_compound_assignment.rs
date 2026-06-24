#[cfg(test)]
#[test]
fn parser_parse_compound_assignment() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse("a += 5", &ParseOptions::default());

    assert!(!block.is_null());

    let block_ref = unsafe { &*block };
    assert_eq!(block_ref.body.size, 1);

    let first_stat = unsafe { *block_ref.body.data.add(0) };
    assert!(unsafe { &*first_stat }.base.is::<AstStatCompoundAssign>());

    let compound_assign = unsafe { &*first_stat }
        .base
        .as_item::<AstStatCompoundAssign>();
    assert!(!compound_assign.is_null());
    assert_eq!(
        unsafe { &*compound_assign }.op,
        luaur_ast::records::ast_expr_binary::AstExprBinary_Op::Add
    );
}
