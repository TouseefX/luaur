//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:788:type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true

#[cfg(test)]
#[test]
fn type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {"hello"}
        local v = t[2]
        if type(v) == "nil" then
            local foo = v
        else
            local foo = v
        end

        if not (type(v) ~= "nil") then
            local foo = v
        else
            local foo = v
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "nil & string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(4, 24))
            )
        );
        assert_eq!(
            "string & ~nil",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(6, 24))
            )
        );

        assert_eq!(
            "nil & string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(10, 24))
            )
        );
        assert_eq!(
            "string & ~nil",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(12, 24))
            )
        );
    } else {
        assert_eq!(
            "nil",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(4, 24))
            )
        );
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(6, 24))
            )
        );

        assert_eq!(
            "nil",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(10, 24))
            )
        );
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(12, 24))
            )
        );
    }
}
