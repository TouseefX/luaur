//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:285:type_infer_provisional_discriminate_from_x_not_equal_to_nil`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_provisional_discriminate_from_x_not_equal_to_nil

#[cfg(test)]
#[test]
fn type_infer_provisional_discriminate_from_x_not_equal_to_nil() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = {x: string, y: number} | {x: nil, y: nil}

        local function f(t: T)
            if t.x ~= nil then
                local foo = t
            else
                local bar = t
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "{ x: string, y: number }",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
        );
        assert_eq!(
            "{ x: nil, y: nil }",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
        );
    } else {
        assert_eq!(
            "{ x: string, y: number }",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
        );
        assert_eq!(
            "{ x: nil, y: nil } | { x: string, y: number }",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
        );
    }
}
