//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:747:linter_implicit_return`
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
//!   - translates_to -> rust_item linter_implicit_return

#[cfg(test)]
#[test]
fn linter_implicit_return() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
--!nonstrict
function f1(a)
    if not a then
        return 5
    end
end

function f2(a)
    if not a then
        return
    end
end

function f3(a)
    if not a then
        return 5
    else
        return
    end
end

function f4(a)
    for i in pairs(a) do
        if i > 5 then
            return i
        end
    end

    print("element not found")
end

function f5(a)
    for i in pairs(a) do
        if i > 5 then
            return i
        end
    end

    error("element not found")
end

f6 = function(a)
    if a == 0 then
        return 42
    end
end

function f7(a)
    repeat
        return 10
    until a ~= nil
end

return f1,f2,f3,f4,f5,f6,f7
"#,
        ),
        None,
    );

    assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(5, result.warnings[0].location.begin.line);
    assert_eq!(
        "Function 'f1' can implicitly return no values even though there's an explicit return at line 5; add explicit return to silence",
        result.warnings[0].text
    );
    assert_eq!(29, result.warnings[1].location.begin.line);
    assert_eq!(
        "Function 'f4' can implicitly return no values even though there's an explicit return at line 26; add explicit return to silence",
        result.warnings[1].text
    );
    assert_eq!(45, result.warnings[2].location.begin.line);
    assert_eq!(
        "Function can implicitly return no values even though there's an explicit return at line 45; add explicit return to silence",
        result.warnings[2].text
    );
}
