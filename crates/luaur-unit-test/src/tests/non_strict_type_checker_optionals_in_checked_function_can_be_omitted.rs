//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_optionals_in_checked_function_can_be_omitted() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
optionalArgsAtTheEnd1("a")
optionalArgsAtTheEnd1("a", 3)
optionalArgsAtTheEnd1("a", nil, 3)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
