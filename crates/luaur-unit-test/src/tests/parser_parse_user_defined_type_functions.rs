#[cfg(test)]
#[test]
fn parser_parse_user_defined_type_functions() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "type function foo()\n\
         return types.number\n\
         end\n\
         \n\
         export type function bar()\n\
         return types.string\n\
         end",
        &ParseOptions::default(),
    );
    let root = unsafe { &*block };
    assert!(!root.body.data.is_null());

    let first_stat = unsafe { *root.body.data.add(0) };
    let type_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!type_function.is_null());

    let f = unsafe { &*type_function };
    // C++ `CHECK(f->name == "foo")` — AstName == "literal" is strcmp on content.
    assert_eq!(unsafe { core::ffi::CStr::from_ptr(f.name.value) }, c"foo");
}
