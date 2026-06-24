//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:824:linter_implicit_return_infinite_loop`
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
//!   - translates_to -> rust_item linter_implicit_return_infinite_loop

#[cfg(test)]
#[test]
fn linter_implicit_return_infinite_loop() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
--!nonstrict
function f1(a)
    while true do
        if math.random() > 0.5 then
            return 5
        end
    end
end

function f2(a)
    repeat
        if math.random() > 0.5 then
            return 5
        end
    until false
end

function f3(a)
    while true do
        if math.random() > 0.5 then
            return 5
        end
        if math.random() < 0.1 then
            break
        end
    end
end

function f4(a)
    repeat
        if math.random() > 0.5 then
            return 5
        end
        if math.random() < 0.1 then
            break
        end
    until false
end

return f1,f2,f3,f4
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(26, result.warnings[0].location.begin.line);
    assert_eq!(
        "Function 'f3' can implicitly return no values even though there's an explicit return at line 22; add explicit return to silence",
        result.warnings[0].text
    );
    assert_eq!(37, result.warnings[1].location.begin.line);
    assert_eq!(
        "Function 'f4' can implicitly return no values even though there's an explicit return at line 33; add explicit return to silence",
        result.warnings[1].text
    );
}
