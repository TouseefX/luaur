#[cfg(test)]
#[test]
fn parser_parse_attribute_on_local_function_stat() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::{AstAttr, AstAttrType};
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    // C++ R"(\n    @checked\nlocal function hello(x, y)\n    return x + y\nend)" —
    // `@checked` is one token at column 4 (4-space indent). The port split it across
    // lines and dropped the indent.
    let code = "\n    @checked\nlocal function hello(x, y)\n    return x + y\nend";

    let stat: *mut AstStatBlock = fix.parse(
        code,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert!(!stat.is_null());

    let stat_fun: *mut AstStatLocalFunction = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocalFunction>(
            *(*stat).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!stat_fun.is_null());

    let attributes: AstArray<*mut AstAttr> = unsafe { (*(*stat_fun).func).attributes };

    assert_eq!(attributes.size, 1);

    check_attribute(
        unsafe { &**attributes.data.add(0) },
        AstAttrType::Checked,
        Location::new(Position::new(1, 4), Position::new(1, 12)),
    );
}
