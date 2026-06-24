#[cfg(test)]
#[test]
fn parser_type_alias_should_work_when_name_is_also_local() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local A = nil\n        type A = string",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());
    let root = unsafe { &*block };
    assert_eq!(root.body.size, 2);
    let first_stat = unsafe { *root.body.data.add(0) };
    assert!(unsafe { &*first_stat }
        .base
        .is::<luaur_ast::records::ast_stat_local::AstStatLocal>());
    let second_stat = unsafe { *root.body.data.add(1) };
    assert!(unsafe { &*second_stat }
        .base
        .is::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>());
}
