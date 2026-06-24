//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1365:linter_dead_locals_used`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
//!   - translates_to -> rust_item linter_dead_locals_used

#[cfg(test)]
#[test]
fn linter_dead_locals_used() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
--!nolint LocalShadow
do
    local x
    for x in pairs({}) do
        print(x)
    end
    print(x) -- x is not initialized
end

do
    local a, b, c = 1, 2
    print(a, b, c) -- c is not initialized
end

do
    local a, b, c = table.unpack({})
    print(a, b, c) -- no warning as we don't know anything about c
end
    "#,
        ),
        None,
    );

    assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Variable 'x' defined at line 4 is never initialized or assigned; initialize with 'nil' to silence",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Assigning 2 values to 3 variables initializes extra variables with nil; add 'nil' to value list to silence",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Variable 'c' defined at line 12 is never initialized or assigned; initialize with 'nil' to silence",
        result.warnings[2].text.as_str()
    );
}
