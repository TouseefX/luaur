//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3153:type_infer_refinements_cli_181100_fast_track_refinement_against_unknown`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_refinements_cli_181100_fast_track_refinement_against_unknown

#[cfg(test)]
#[test]
fn type_infer_refinements_cli_181100_fast_track_refinement_against_unknown() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local Class = {}
        Class.__index = Class

        type Class = setmetatable<{ A: number }, typeof(Class)>

        function Class.Foo(x: Class, y: Class, z: Class)
            if y == z then
                return
            end
            local bar = y.A
            print(bar)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(13, 19))
        )
    );
}
