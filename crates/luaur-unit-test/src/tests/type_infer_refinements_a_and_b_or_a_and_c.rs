//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:276:type_infer_refinements_a_and_b_or_a_and_c`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_a_and_b_or_a_and_c

#[cfg(test)]
#[test]
fn type_infer_refinements_a_and_b_or_a_and_c() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(a: string?, b: number?, c: boolean)
            if (a and b) or (a and c) then
                local foo = a
                local bar = b
                local baz = c
            else
                local foo = a
                local bar = b
                local baz = c
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "boolean",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
        );
    } else {
        assert_eq!(
            "true",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
        );
    }
    assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 28)))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
    );
}
