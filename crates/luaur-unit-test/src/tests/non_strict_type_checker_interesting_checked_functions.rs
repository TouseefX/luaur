//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_interesting_checked_functions() {
    use crate::functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_ast::records::position::Position;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
onlyNums(1,1,1)
onlyNums(1, "a")

mixedArgs("a", 1, 2)
mixedArgs(1, 1, 1)
mixedArgs("a", true)

optionalArg(nil)
optionalArg("a")
optionalArg(3)
"#,
    ));

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 12), "onlyNums");
    require_non_strict_checked_error_at(&result, Position::new(5, 10), "mixedArgs");
    require_non_strict_checked_error_at(&result, Position::new(6, 15), "mixedArgs");
    require_non_strict_checked_error_at(&result, Position::new(10, 12), "optionalArg");
}
