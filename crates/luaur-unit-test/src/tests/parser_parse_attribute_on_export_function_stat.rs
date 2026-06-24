#[cfg(test)]
#[test]
fn parser_parse_attribute_on_export_function_stat() {
    use crate::functions::check_attribute::check_attribute;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source = alloc::string::String::from(
        "\n@checked\nexport function hello(x, y)\n    return x + y\nend",
    );
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();
    let stat = fixture.parse(&source, &parse_options);

    assert!(!stat.is_null(), "parse should succeed");

    let first_stat = unsafe { (*stat).body.data.add(0) };
    let stat_fun = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocalFunction>(
            *first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(
        !stat_fun.is_null(),
        "first stat should be AstStatLocalFunction"
    );

    assert_eq!(
        unsafe { (*stat_fun).base.base.location.begin },
        Position::new(1, 0)
    );
    assert!(unsafe { (*stat_fun).name.as_ref().unwrap().is_exported });
    assert!(unsafe { (*stat_fun).name.as_ref().unwrap().is_const });

    let func = unsafe { (*stat_fun).func };
    let attributes = unsafe { (*func).attributes };

    assert_eq!(attributes.size, 1);

    let attr = unsafe { *attributes.data.add(0) };
    check_attribute(
        unsafe { &*attr },
        luaur_ast::records::ast_attr::AstAttrType::Checked,
        Location::new(Position::new(1, 0), Position::new(1, 8)),
    );
}
