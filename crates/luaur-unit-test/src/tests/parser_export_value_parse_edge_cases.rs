#[cfg(test)]
#[test]
fn parser_export_value_parse_edge_cases() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source1 = alloc::string::String::from("export = 5\nexport += 1\nexport()");
    let parse_result1 = fixture.parse(
        &source1,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let ast_block1 = unsafe { &*parse_result1 };
    assert_eq!(ast_block1.body.size, 3);
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_assign::AstStatAssign>(
            *ast_block1.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign,
        >(*ast_block1.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode)
        .is_null()
    });
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_expr::AstStatExpr>(
            *ast_block1.body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });

    let source2 = alloc::string::String::from("export local x = 5");
    let _parse_result2 = fixture.parse(
        &source2,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let source3 = alloc::string::String::from("export const x = 5");
    let _parse_result3 = fixture.parse(
        &source3,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let source4 = alloc::string::String::from("export function foo()\nend");
    let _parse_result4 = fixture.parse(
        &source4,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    fixture.match_parse_error(
        &alloc::string::String::from("export 42"),
        &alloc::string::String::from(
            "Incomplete statement: expected assignment or a function call",
        ),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("export if true then end"),
        &alloc::string::String::from(
            "Incomplete statement: expected assignment or a function call",
        ),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("export"),
        &alloc::string::String::from(
            "Incomplete statement: expected assignment or a function call",
        ),
        None,
    );
}
