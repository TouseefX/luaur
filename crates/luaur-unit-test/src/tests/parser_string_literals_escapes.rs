#[cfg(test)]
#[test]
fn parser_string_literals_escapes() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return\n\
         \"\\xAB\",\n\
         \"\\u{2024}\",\n\
         \"\\121\",\n\
         \"\\1x\",\n\
         \"\\t\",\n\
         \"\\n\"",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()`: the return is the first body
    // element, not the block's own base node.
    let ret = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(
            *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ret.is_null());
    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 6);

    let check_str = |idx: usize, expected: &[u8]| {
        let expr = unsafe { *ret.list.data.add(idx) };
        let str_node = unsafe { (*expr).base.as_item::<AstExprConstantString>() };
        assert!(!str_node.is_null());
        let str_node = unsafe { &*str_node };
        let bytes = unsafe {
            core::slice::from_raw_parts(
                str_node.value.data as *const u8,
                str_node.value.size as usize,
            )
        };
        assert_eq!(bytes, expected);
    };

    check_str(0, &[0xAB]);
    check_str(1, &[0xE2, 0x80, 0xA4]);
    check_str(2, &[0x79]);
    check_str(3, &[0x01, b'x']);
    check_str(4, &[b'\t']);
    check_str(5, &[b'\n']);
}
