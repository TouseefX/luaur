#[cfg(test)]
#[test]
fn parser_string_literals_escape_newline() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return \"foo\\z\n   bar\", \"foo\\\n    bar\", \"foo\\\r\nbar\"",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()`: deref the first body slot
    // and RTTI-cast it. The port cast the array pointer itself to *AstStatReturn.
    let ret = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(
            *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ret.is_null());

    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 3);

    let str0 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!str0.is_null());
    let str0 = unsafe { &*str0 };
    let s0 = unsafe {
        core::slice::from_raw_parts(str0.value.data as *const u8, str0.value.size as usize)
    };
    assert_eq!(core::str::from_utf8(s0).unwrap(), "foobar");

    let str1 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!str1.is_null());
    let str1 = unsafe { &*str1 };
    let s1 = unsafe {
        core::slice::from_raw_parts(str1.value.data as *const u8, str1.value.size as usize)
    };
    assert_eq!(core::str::from_utf8(s1).unwrap(), "foo\n    bar");

    let str2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(2) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!str2.is_null());
    let str2 = unsafe { &*str2 };
    let s2 = unsafe {
        core::slice::from_raw_parts(str2.value.data as *const u8, str2.value.size as usize)
    };
    assert_eq!(core::str::from_utf8(s2).unwrap(), "foo\nbar");
}
