//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1373:type_infer_refinements_narrow_boolean_to_true_or_false`
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
//!   - translates_to -> rust_item type_infer_refinements_narrow_boolean_to_true_or_false

#[cfg(test)]
#[test]
fn type_infer_refinements_narrow_boolean_to_true_or_false() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: boolean)
            if x then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "true",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
        "false",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
}
