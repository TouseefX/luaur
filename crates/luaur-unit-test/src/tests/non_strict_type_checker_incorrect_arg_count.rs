//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_incorrect_arg_count() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_analysis::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
foo.bar(1,2,3)
abs(3, "hi");
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[0])
        .expect("expected CheckedFunctionIncorrectArgs");
    let r2 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[1])
        .expect("expected CheckedFunctionIncorrectArgs");

    assert_eq!("abs", r1.functionName());
    assert_eq!("foo.bar", r2.functionName());
}
