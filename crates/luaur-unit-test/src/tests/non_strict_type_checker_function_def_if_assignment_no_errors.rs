//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_function_def_if_assignment_no_errors() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
function f(x : string | number)
    if cond() then
        x = 5
    else
        x = "hi"
    end
    abs(x)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
