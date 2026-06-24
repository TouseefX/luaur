//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:474:linter_unreachable_code_if_merge`
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
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item linter_unreachable_code_if_merge

#[cfg(test)]
#[test]
fn linter_unreachable_code_if_merge() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
function foo1(a)
    if a then
        return 'x'
    else
        return 'y'
    end
    return 'z'
end

function foo2(a)
    if a then
        return 'x'
    end
    return 'z'
end

function foo3(a)
    if a then
        return 'x'
    else
        print('y')
    end
    return 'z'
end

return { foo1, foo2, foo3 }
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(7, result.warnings[0].location.begin.line);
    assert_eq!(
        "Unreachable code (previous statement always returns)",
        result.warnings[0].text.as_str()
    );
}
