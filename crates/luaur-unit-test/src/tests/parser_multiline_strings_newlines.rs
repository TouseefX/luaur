#[cfg(test)]
#[test]
fn parser_multiline_strings_newlines() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return [=[\nfoo\r\nbar\n\nbaz\n]=]",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let stat_block = unsafe { &*stat };
    assert!(!stat_block.body.data.is_null());

    let first_stat = unsafe { *stat_block.body.data };
    assert!(!first_stat.is_null());

    let ret = unsafe { (*first_stat).base.as_item::<AstStatReturn>() };
    assert!(!ret.is_null());

    let ret = unsafe { &*ret };
    assert!(!ret.list.data.is_null());

    let first_expr = unsafe { *ret.list.data };
    assert!(!first_expr.is_null());

    let str_expr = unsafe { (*first_expr).base.as_item::<AstExprConstantString>() };
    assert!(!str_expr.is_null());

    let str_expr = unsafe { &*str_expr };
    let s = unsafe {
        core::slice::from_raw_parts(
            str_expr.value.data as *const u8,
            str_expr.value.size as usize,
        )
    };
    let actual = core::str::from_utf8(s).unwrap();
    assert_eq!(actual, "foo\nbar\n\nbaz\n");
}
