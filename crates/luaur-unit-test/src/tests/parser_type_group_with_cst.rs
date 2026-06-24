#[cfg(test)]
#[test]
fn parser_type_group_with_cst() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
    use luaur_ast::records::ast_type_group::AstTypeGroup;
    use luaur_ast::records::cst_type_group::CstTypeGroup;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let _ff = ScopedFastFlag::new(&luaur_common::FFlag::LuauCstTypeGroup, true);

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n        type t = (number)\n    ");
    let mut parse_options = ParseOptions::parse_options();
    parse_options.store_cst_data = true;
    let result = fix.parse_ex(&code, &parse_options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let type_alias = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
            *(*result.root).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!type_alias.is_null());

    let group = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeGroup>(
            (*type_alias).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!group.is_null());

    let base_cst_node = result
        .cst_node_map
        .find(&(group as *mut luaur_ast::records::ast_node::AstNode));
    assert!(base_cst_node.is_some());

    let cst_node = unsafe { luaur_ast::rtti::cst_node_as::<CstTypeGroup>(*base_cst_node.unwrap()) };
    assert!(!cst_node.is_null());

    assert_eq!(Position::new(1, 24), unsafe { (*cst_node).close_position });
}
