//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:831:non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed`
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
//!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed

#[cfg(test)]
#[test]
fn non_strict_type_checker_new_non_strict_stringifies_checked_function_errors_as_one_indexed() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_analysis::records::checked_function_call_error::CheckedFunctionCallError;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
getAllTheArgsWrong(3, true, "what")
"#,
    ));

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[0]).is_some());
    assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[1]).is_some());
    assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[2]).is_some());
    assert_eq!(
        "the function 'getAllTheArgsWrong' expects to get a string as its 1st argument, but is being given a number",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "the function 'getAllTheArgsWrong' expects to get a number as its 2nd argument, but is being given a boolean",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "the function 'getAllTheArgsWrong' expects to get a boolean as its 3rd argument, but is being given a string",
        to_string_type_error(&result.errors[2])
    );
}
