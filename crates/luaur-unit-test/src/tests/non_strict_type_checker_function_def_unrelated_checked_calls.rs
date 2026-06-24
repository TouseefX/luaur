//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:326:non_strict_type_checker_function_def_unrelated_checked_calls`
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
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_function_def_unrelated_checked_calls

#[cfg(test)]
#[test]
fn non_strict_type_checker_function_def_unrelated_checked_calls() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
function h(x, y)
    abs(x)
    lower(y)
end
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
