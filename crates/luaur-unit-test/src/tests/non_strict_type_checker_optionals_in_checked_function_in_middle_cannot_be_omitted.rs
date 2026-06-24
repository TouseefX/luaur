//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_optionals_in_checked_function_in_middle_cannot_be_omitted() {
    use crate::functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at;
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_analysis::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs;
    use luaur_ast::records::position::Position;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
optionalArgsAtTheEnd2("a", "a") -- error
optionalArgsAtTheEnd2("a", nil, "b")
optionalArgsAtTheEnd2("a", 3, "b")
optionalArgsAtTheEnd2("a", "b", "c") -- error
"#,
    ));

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(1, 27), "optionalArgsAtTheEnd2");
    require_non_strict_checked_error_at(&result, Position::new(4, 27), "optionalArgsAtTheEnd2");

    let r1 = type_error_data_ref::<CheckedFunctionIncorrectArgs>(&result.errors[2])
        .expect("expected CheckedFunctionIncorrectArgs");

    assert_eq!(3, r1.expected());
    assert_eq!(2, r1.actual());
}
