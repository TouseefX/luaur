#[cfg(test)]
#[test]
fn parser_parse_parametrized_attribute_on_function_stat() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::{AstAttr, AstAttrType};
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        r#"
@[deprecated{ use = "greetng", reason = "Using <hello> is too causal"}]
function hello(x, y)
    return x + y
end"#,
    );

    let result = fix.parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let stat_block: *mut AstStatBlock = result;
    assert!(!stat_block.is_null());

    let first_stat: *mut AstStat = unsafe { *(*stat_block).body.data.add(0) };
    let stat_fun: *mut AstStatFunction = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_fun.is_null());

    let attributes: AstArray<*mut AstAttr> = unsafe {
        (*luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_function::AstExprFunction>(
            (*stat_fun).func as *mut luaur_ast::records::ast_node::AstNode,
        ))
        .attributes
    };

    assert_eq!(attributes.size, 1);

    let attr: *mut AstAttr = unsafe { *attributes.data.add(0) };
    let expected_location = Location::new(Position::new(1, 2), Position::new(1, 70));
    check_attribute(
        unsafe { &*attr },
        AstAttrType::Deprecated,
        expected_location,
    );
}
