#[cfg(test)]
#[test]
fn parser_parse_attribute_on_function_type_declaration() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_attr::AstAttrType;
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n@checked declare function abs(n: number): number\n");
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 0);

    let root_block = unsafe { &*result.root };
    assert_eq!(root_block.body.size, 1);

    let root = unsafe { &**root_block.body.data };
    let func = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatDeclareFunction>(
            root as *const AstStat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!func.is_null());

    assert!(unsafe { (*func).is_checked_function() });

    let attributes = unsafe { (*func).attributes };
    assert_eq!(attributes.size, 1);

    let attr = unsafe { &**attributes.data };
    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 8));
    crate::functions::check_attribute::check_attribute(
        attr,
        AstAttrType::Checked,
        expected_location,
    );
}
