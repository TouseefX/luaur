#[cfg(test)]
#[test]
fn parser_parse_const() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
    use luaur_ast::records::ast_local::AstLocal;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::default();
    let _sff = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let stat = fixture.parse("const f = 42", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());
    assert_eq!(root.body.size, 1);
    let first_stat = unsafe { *root.body.data.add(0) };
    assert!(unsafe { (*first_stat).base.is::<AstStatLocal>() });
    let stat_local = unsafe { (*first_stat).base.as_item::<AstStatLocal>() };
    assert!(!stat_local.is_null());
    let stat_local = unsafe { &*stat_local };
    assert_eq!(stat_local.vars.size, 1);
    assert_eq!(stat_local.values.size, 1);
    let local = unsafe { *stat_local.vars.data.add(0) };
    assert!(!local.is_null());
    let local = unsafe { &*local };
    let name_str = unsafe { core::ffi::CStr::from_ptr(local.name.value).to_string_lossy() };
    assert_eq!(name_str, "f");
    assert!(local.is_const);
    let value = unsafe { *stat_local.values.data.add(0) };
    assert!(!value.is_null());
    assert!(unsafe { (*value).base.is::<AstExprConstantNumber>() });
    let expr_const = unsafe { (*value).base.as_item::<AstExprConstantNumber>() };
    assert!(!expr_const.is_null());
    let expr_const = unsafe { &*expr_const };
    assert_eq!(expr_const.value, 42.0);
}
