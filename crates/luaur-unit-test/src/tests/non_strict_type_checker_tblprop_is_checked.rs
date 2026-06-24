//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_tblprop_is_checked() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_ast::records::position::Position;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
foo.bar("hi")
"#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    crate::functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at(
        &result,
        Position::new(1, 8),
        "foo.bar",
    );
}
