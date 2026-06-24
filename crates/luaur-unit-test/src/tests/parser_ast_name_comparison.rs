#[cfg(test)]
#[test]
fn parser_ast_name_comparison() {
    use luaur_ast::records::ast_name::AstName;

    let empty1 = AstName::new();
    let empty2 = AstName::new();
    assert!(!empty1.operator_lt(&empty2));

    let one = AstName::ast_name_c_char(c"one".as_ptr() as *const core::ffi::c_char);
    let two = AstName::ast_name_c_char(c"two".as_ptr() as *const core::ffi::c_char);

    let one_lt_two = one.operator_lt(&two);
    let two_lt_one = two.operator_lt(&one);
    assert_ne!(one_lt_two, two_lt_one);
}
