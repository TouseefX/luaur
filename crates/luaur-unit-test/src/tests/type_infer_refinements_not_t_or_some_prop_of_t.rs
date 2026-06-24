//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1038:type_infer_refinements_not_t_or_some_prop_of_t`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_not_t_or_some_prop_of_t

#[cfg(test)]
#[test]
fn type_infer_refinements_not_t_or_some_prop_of_t() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t: {x: boolean}?)
            if not t or t.x then
                local foo = t
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "({ read x: ~(false?) } & { x: boolean })?",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
        );
    } else {
        assert_eq!(
            "{ x: boolean }?",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
        );
    }
}
