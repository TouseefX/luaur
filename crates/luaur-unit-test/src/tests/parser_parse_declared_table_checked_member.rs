#[cfg(test)]
#[test]
fn parser_parse_declared_table_checked_member() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_table::AstTypeTable;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    // Faithful to the C++ R"BUILTIN_SRC(...)" raw string: leading newline, then
    // `    declare math : {` (4 spaces), `        abs : ...` (8 spaces), then `}`.
    let code = alloc::string::String::from(
        "\n    declare math : {\n        abs : @checked (number) -> number\n}\n",
    );
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let pr = fix.try_parse(&code, &opts);
    luaur_common::LUAU_ASSERT!(pr.errors.is_empty());

    luaur_common::LUAU_ASSERT!(unsafe { (*pr.root).body.size } == 1);
    // C++: `AstStat* root = *(pr.root->body.data);` — the FIRST statement, not the block.
    let root = unsafe { *(*pr.root).body.data.add(0) };
    let glob =
        unsafe { luaur_ast::rtti::ast_node_as::<AstStatDeclareGlobal>(root as *mut AstNode) };
    luaur_common::LUAU_ASSERT!(glob.is_null() == false);
    let glob = unsafe { &*glob };
    let tbl = unsafe { luaur_ast::rtti::ast_node_as::<AstTypeTable>(glob.type_ as *mut AstNode) };
    luaur_common::LUAU_ASSERT!(tbl.is_null() == false);
    let tbl = unsafe { &*tbl };
    luaur_common::LUAU_ASSERT!(tbl.props.size == 1);
    let prop = unsafe { &*tbl.props.data };
    let func =
        unsafe { luaur_ast::rtti::ast_node_as::<AstTypeFunction>(prop.r#type as *mut AstNode) };
    luaur_common::LUAU_ASSERT!(func.is_null() == false);
    let func = unsafe { &*func };
    luaur_common::LUAU_ASSERT!(func.is_checked_function());
}
