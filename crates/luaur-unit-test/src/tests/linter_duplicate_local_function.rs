//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1422:linter_duplicate_local_function`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
//!   - calls -> method LintOptions::setDefaults (Config/src/LinterConfig.cpp)
//!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item linter_duplicate_local_function

#[cfg(test)]
#[test]
fn linter_duplicate_local_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_config::enums::code::Code;
    use luaur_config::records::lint_options::LintOptions;

    let mut options = LintOptions::default();
    options.set_defaults();
    options.enable_warning(Code::Code_DuplicateFunction);
    options.enable_warning(Code::Code_LocalShadow);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
        local function x() end

        print(x)

        local function x() end

        return x
    "#,
        ),
        Some(options),
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(Code::Code_DuplicateFunction, result.warnings[0].code);
}
