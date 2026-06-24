#[cfg(test)]
#[test]
fn parser_parse_top_level_checked_fn() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("@checked declare function abs(n: number): number\n");
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 0);

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let root = unsafe { *(*result.root).body.data };
    let func = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction,
        >(root as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!func.is_null());
    assert!(unsafe { (*func).is_checked_function() });
}
