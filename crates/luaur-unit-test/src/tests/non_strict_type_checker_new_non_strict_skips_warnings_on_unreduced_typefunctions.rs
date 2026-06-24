//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_new_non_strict_skips_warnings_on_unreduced_typefunctions() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
function foo(x)
    local y = x + 1
    return abs(y)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
