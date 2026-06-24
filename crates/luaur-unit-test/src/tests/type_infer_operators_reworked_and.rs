//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1368:type_infer_operators_reworked_and`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_operators_reworked_and

#[cfg(test)]
#[test]
fn type_infer_operators_reworked_and() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local a: number? = 5
local b: boolean = (a or 1) > 10
local c -- free

local x = a and 1
local y = 'a' and 1
local z = b and 1
local w = c and 1
    "#,
        ),
        None,
    );

    assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "false | number",
            to_string_type_id(fixture.base.require_type_string(&String::from("z")))
        );
        assert_eq!(
            "number?",
            to_string_type_id(fixture.base.require_type_string(&String::from("w")))
        );
    } else {
        assert_eq!(
            "boolean | number",
            to_string_type_id(fixture.base.require_type_string(&String::from("z")))
        );
        assert_eq!(
            "(boolean | number)?",
            to_string_type_id(fixture.base.require_type_string(&String::from("w")))
        );
    }
}
