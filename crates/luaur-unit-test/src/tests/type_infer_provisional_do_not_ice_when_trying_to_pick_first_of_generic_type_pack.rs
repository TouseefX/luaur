//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:362:type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack

#[cfg(test)]
#[test]
fn type_infer_provisional_do_not_ice_when_trying_to_pick_first_of_generic_type_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f() end

        local g = function() return f() end

        local x = (f()) -- should error: no return values to assign from the call to f
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "() -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
        assert_eq!(
            "() -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("g")))
        );
        assert_eq!(
            "nil",
            to_string_type_id(fixture.require_type_string(&String::from("x")))
        );
    } else {
        assert_eq!(
            "() -> (a...)",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
        assert_eq!(
            "<a...>() -> (a...)",
            to_string_type_id(fixture.require_type_string(&String::from("g")))
        );
        assert_eq!(
            "any",
            to_string_type_id(fixture.require_type_string(&String::from("x")))
        );
    }
}
