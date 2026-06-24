//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1590:linter_deprecated_api_fenv`
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
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - translates_to -> rust_item linter_deprecated_api_fenv

#[cfg(test)]
#[test]
fn linter_deprecated_api_fenv() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
        &String::from(
            r#"
local f, g, h = ...

getfenv(1)
getfenv(f :: () -> ())
getfenv(g :: number)
getfenv(h :: any)

setfenv(1, {})
setfenv(f :: () -> (), {})
setfenv(g :: number, {})
setfenv(h :: any, {})
"#,
        ),
        None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Function 'getfenv' is deprecated; consider using 'debug.info' instead",
        result.warnings[0].text.as_str()
    );
    assert_eq!(4, result.warnings[0].location.begin.line + 1);
    assert_eq!(
        "Function 'getfenv' is deprecated; consider using 'debug.info' instead",
        result.warnings[1].text.as_str()
    );
    assert_eq!(6, result.warnings[1].location.begin.line + 1);
    assert_eq!(
        "Function 'setfenv' is deprecated",
        result.warnings[2].text.as_str()
    );
    assert_eq!(9, result.warnings[2].location.begin.line + 1);
    assert_eq!(
        "Function 'setfenv' is deprecated",
        result.warnings[3].text.as_str()
    );
    assert_eq!(11, result.warnings[3].location.begin.line + 1);
}
