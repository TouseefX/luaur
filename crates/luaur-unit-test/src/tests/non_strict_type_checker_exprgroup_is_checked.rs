//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:546:non_strict_type_checker_exprgroup_is_checked`
//! Source: `tests/NonStrictTypeChecker.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_exprgroup_is_checked

#[cfg(test)]
#[test]
fn non_strict_type_checker_exprgroup_is_checked() {
    use crate::functions::require_checked_function_call_error::require_checked_function_call_error;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
        local foo = (abs("foo"))
    "#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    require_checked_function_call_error(&result, 0, "abs", "number", "string");
}
