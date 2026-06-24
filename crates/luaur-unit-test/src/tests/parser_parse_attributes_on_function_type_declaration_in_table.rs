#[cfg(test)]
#[test]
fn parser_parse_attributes_on_function_type_declaration_in_table() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
    use luaur_ast::records::ast_table_prop::AstTableProp;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_table::AstTypeTable;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\ndeclare bit32: {\n    band: @checked (...number) -> number\n}",
    );
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let result = fixture.try_parse(&source, &opts);

    assert_eq!(result.errors.len(), 0);

    let root_block = unsafe { &*result.root };
    assert_eq!(root_block.body.size, 1);

    let root_stat = unsafe { &*(*root_block.body.data.add(0)) as *const AstStat };
    let glob = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatDeclareGlobal>(
            root_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!glob.is_null());

    let tbl = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeTable>(
            (*glob).type_ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!tbl.is_null());

    assert_eq!(unsafe { (*tbl).props.size }, 1);
    let prop = unsafe { &*(*tbl).props.data.add(0) };

    let func = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            prop.r#type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!func.is_null());

    let attributes = unsafe { &(*func).attributes };

    assert_eq!(attributes.size, 1);
    let attr = unsafe { &**attributes.data.add(0) };

    check_attribute(
        attr,
        luaur_ast::records::ast_attr::AstAttrType::Checked,
        Location::new(Position::new(2, 10), Position::new(2, 18)),
    );
}
