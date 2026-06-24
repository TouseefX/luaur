#[cfg(test)]
#[test]
fn parser_local_with_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local foo: string = \"Hello Types!\"",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let first_stat = unsafe { *root.body.data.add(0) };
    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local.is_null());

    let local = unsafe { &*local };
    assert_eq!(local.vars.size, 1);

    let l = unsafe { *local.vars.data.add(0) };
    let l_ref = unsafe { &*l };
    assert!(!l_ref.annotation.is_null());

    assert_eq!(local.values.size, 1);

    let code = "local foo: string = \"Hello Types!\"";
    let loc = l_ref.location;
    let name = crate::functions::string_at_location::string_at_location(code, &loc);
    assert_eq!(name, "foo");
}
