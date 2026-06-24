#[cfg(test)]
#[test]
fn parser_parse_debugnoinline_on_local_function() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag::DebugLuauNoInline;

    let _no_inline = ScopedFastFlag::new(&DebugLuauNoInline, true);

    let mut fix = Fixture::default();
    let source = "\n    @debugnoinline\nlocal function hello(x, y)\n    return x + y\nend";

    let stat = fix.parse(
        source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert!(!stat.is_null());

    let stat_local_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocalFunction>(
            *(*stat).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!stat_local_function.is_null());

    let attributes = unsafe { (*(*stat_local_function).func).attributes };

    assert_eq!(attributes.size, 1);

    check_attribute(
        unsafe { &**attributes.data },
        luaur_ast::records::ast_attr::AstAttrType::DebugNoinline,
        Location::new(Position::new(1, 4), Position::new(1, 18)),
    );
}
