#[cfg(test)]
#[test]
fn parser_class_indexer() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_name::AstName;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
    use luaur_ast::records::ast_table_indexer::AstTableIndexer;
    use luaur_ast::records::ast_type_reference::AstTypeReference;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::rtti::ast_node_as;
    use luaur_ast::rtti::ast_node_is;

    let mut fixture = Fixture::default();
    let parse_result = fixture.parse_ex(
        &alloc::string::String::from(
            "declare class Foo\n\
             prop: boolean\n\
             [string]: number\n\
             end",
        ),
        &ParseOptions::default(),
    );
    let root = unsafe { &*parse_result.root };
    assert_eq!(root.body.size, 1);

    let stat_declare_extern_type = unsafe {
        let stat_ptr = *root.body.data.add(0);
        ast_node_as::<AstStatDeclareExternType>(
            stat_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_declare_extern_type.is_null());
    let stat_declare_extern_type = unsafe { &*stat_declare_extern_type };
    assert!(!stat_declare_extern_type.indexer.is_null());
    let indexer = unsafe { &*stat_declare_extern_type.indexer };

    assert!(ast_node_is::<AstTypeReference>(
        indexer.index_type as *mut luaur_ast::records::ast_node::AstNode
    ));
    let index_type_ref = unsafe {
        &*ast_node_as::<AstTypeReference>(
            indexer.index_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(index_type_ref.name.value) },
        c"string"
    );

    assert!(ast_node_is::<AstTypeReference>(
        indexer.result_type as *mut luaur_ast::records::ast_node::AstNode
    ));
    let result_type_ref = unsafe {
        &*ast_node_as::<AstTypeReference>(
            indexer.result_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(result_type_ref.name.value) },
        c"number"
    );

    let error_parse_result = fixture.match_parse_error(
        &alloc::string::String::from(
            "declare class Foo\n\
             [string]: number\n\
             -- can only have one indexer\n\
             [number]: number\n\
             end",
        ),
        &alloc::string::String::from("Cannot have more than one indexer on an extern type"),
        None,
    );
    let error_root = unsafe { &*error_parse_result.root };
    assert_eq!(error_root.body.size, 1);

    let error_stat_declare_extern_type = unsafe {
        let stat_ptr = *error_root.body.data.add(0);
        ast_node_as::<AstStatDeclareExternType>(
            stat_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!error_stat_declare_extern_type.is_null());
    let error_stat_declare_extern_type = unsafe { &*error_stat_declare_extern_type };
    assert!(!error_stat_declare_extern_type.indexer.is_null());
}
