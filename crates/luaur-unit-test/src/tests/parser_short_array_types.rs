#[cfg(test)]
#[test]
fn parser_short_array_types() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: {string}", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let local_stmt = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local_stmt.is_null());

    let annotation = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_table::AstTypeTable>(
            (**(*local_stmt).vars.data.add(0)).annotation
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!annotation.is_null());

    assert_eq!(unsafe { (*annotation).props.size }, 0);
    assert!(!unsafe { (*annotation).indexer }.is_null());

    let indexer = unsafe { &*(*annotation).indexer };
    let index_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            indexer.index_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!index_type.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*index_type).name.value).to_string_lossy() },
        "number"
    );

    let result_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            indexer.result_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!result_type.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*result_type).name.value).to_string_lossy() },
        "string"
    );
}
