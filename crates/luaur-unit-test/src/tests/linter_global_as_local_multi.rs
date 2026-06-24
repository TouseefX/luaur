//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:287:linter_global_as_local_multi`
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
//!   - calls -> function createFunction (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item linter_global_as_local_multi

#[cfg(test)]
#[test]
fn linter_global_as_local_multi() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local createFunction = function(configValue)
    -- Create an internal convenience function
    local function internalLogic()
        print(configValue) -- prints passed-in value
    end
    -- Here, we thought we were creating another internal convenience function
    -- that closed over the passed-in configValue, but this is actually being
    -- declared at module scope!
    function moreInternalLogic()
        print(configValue) -- nil!!!
    end
    return function()
        internalLogic()
        moreInternalLogic()
        return nil
    end
end
fnA = createFunction(true)
fnB = createFunction(false)
fnA() -- prints "true", "nil"
fnB() -- prints "false", "nil"
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Global 'moreInternalLogic' is only used in the enclosing function defined at line 2; consider changing it to local",
        result.warnings[0].text.as_str()
    );
}
