#[cfg(test)]
#[test]
fn parser_short_array_types_do_not_break_field_names() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: {string: number}", &ParseOptions::default());
    assert!(!stat.is_null());
    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            (*stat).body.data.add(0).read() as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local.is_null());
    let annotation = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_table::AstTypeTable>(
            (*(*local).vars.data.add(0).read()).annotation
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!annotation.is_null());
    assert_eq!(unsafe { (*annotation).props.size }, 1);
    assert!(unsafe { (*annotation).indexer }.is_null());
    let prop = unsafe { (*annotation).props.data.add(0).read() };
    let prop_name = unsafe { core::ffi::CStr::from_ptr(prop.name.value).to_string_lossy() };
    assert_eq!(prop_name, "string");
    let prop_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            prop.r#type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!prop_type.is_null());
    let type_name = unsafe { core::ffi::CStr::from_ptr((*prop_type).name.value).to_string_lossy() };
    assert_eq!(type_name, "number");
}
