//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:307:type_infer_refinements_type_assertion_expr_carry_its_constraints`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_refinements_type_assertion_expr_carry_its_constraints

#[cfg(test)]
#[test]
fn type_infer_refinements_type_assertion_expr_carry_its_constraints() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function g(a: number?, b: string?)
            if (a :: any) and (b :: any) then
                local x = a
                local y = b
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "number?",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
        );
        assert_eq!(
            "string?",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
        );
    } else {
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
        );
        assert_eq!(
            "string",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
        );
    }
}
