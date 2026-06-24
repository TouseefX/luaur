#[cfg(test)]
#[test]
fn parser_string_literals_escape() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return\n\
         \"foo\\n\\r\",\n\
         \"foo\\0324\",\n\
         \"foo\\x204\",\n\
         \"foo\\u{20}\",\n\
         \"foo\\u{0451}\"",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let ret = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(
            *(*stat).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!ret.is_null());
    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 5);

    let str0 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert_eq!(
        unsafe {
            core::slice::from_raw_parts(str0.value.data as *const u8, str0.value.size as usize)
        },
        b"foo\n\r"
    );

    let str1 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert_eq!(
        unsafe {
            core::slice::from_raw_parts(str1.value.data as *const u8, str1.value.size as usize)
        },
        b"foo 4"
    );

    let str2 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(2) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert_eq!(
        unsafe {
            core::slice::from_raw_parts(str2.value.data as *const u8, str2.value.size as usize)
        },
        b"foo 4"
    );

    let str3 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(3) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert_eq!(
        unsafe {
            core::slice::from_raw_parts(str3.value.data as *const u8, str3.value.size as usize)
        },
        b"foo "
    );

    let str4 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *ret.list.data.add(4) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert_eq!(
        unsafe {
            core::slice::from_raw_parts(str4.value.data as *const u8, str4.value.size as usize)
        },
        b"foo\xd1\x91"
    );
}
