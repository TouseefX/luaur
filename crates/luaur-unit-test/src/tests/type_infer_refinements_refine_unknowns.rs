//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1743:type_infer_refinements_refine_unknowns`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refine_unknowns

#[cfg(test)]
#[test]
fn type_infer_refinements_refine_unknowns() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: unknown)
            if type(x) == "string" then
                local foo = x
            else
                local bar = x
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(3, 28))
            )
        );
        assert_eq!(
            "~string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
    } else {
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(3, 28))
            )
        );
        assert_eq!(
            "unknown",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
    }
}
