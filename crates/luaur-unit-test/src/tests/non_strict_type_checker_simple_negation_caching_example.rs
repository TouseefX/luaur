//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_simple_negation_caching_example() {
    use crate::functions::require_non_strict_checked_error_at::require_non_strict_checked_error_at;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_ast::records::position::Position;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
local x = 3
abs(x)
abs(x)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
local x = 3
contrived(x)
contrived(x)
"#,
    ));

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    require_non_strict_checked_error_at(&result, Position::new(2, 10), "contrived");
    require_non_strict_checked_error_at(&result, Position::new(3, 10), "contrived");
}
