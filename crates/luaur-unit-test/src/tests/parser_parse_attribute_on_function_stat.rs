#[cfg(test)]
#[test]
fn parser_parse_attribute_on_function_stat() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    // C++ R"(\n@checked\nfunction hello(x, y)\n    return x + y\nend)" — `@checked`
    // is ONE token; the port split `@` and `checked` onto separate lines, so the
    // lexer saw `@` with no following name ("Attribute name is missing").
    let code = "\n@checked\nfunction hello(x, y)\n    return x + y\nend";

    let stat = fix.parse(
        code,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert!(!stat.is_null());

    let stat_fun = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_fun.is_null());

    let attributes = unsafe { (*stat_fun).func.as_ref().unwrap().attributes };

    assert_eq!(attributes.size, 1);

    let attr = unsafe { *attributes.data.add(0) };
    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 8));
    check_attribute(
        unsafe { &*attr },
        luaur_ast::records::ast_attr::AstAttrType::Checked,
        expected_location,
    );
}
