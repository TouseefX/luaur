//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_function_def_basic_no_errors() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
function f(x)
    abs(x)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
