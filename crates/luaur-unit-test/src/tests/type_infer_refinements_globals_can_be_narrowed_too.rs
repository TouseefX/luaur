//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2215:type_infer_refinements_globals_can_be_narrowed_too`
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
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_refinements_globals_can_be_narrowed_too

#[cfg(test)]
#[test]
fn type_infer_refinements_globals_can_be_narrowed_too() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        if typeof(string) == 'string' then
            local foo = string
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "string & typeof(string)",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(2, 24))
            )
        );
    } else {
        assert_eq!(
            "never",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(2, 24))
            )
        );
    }
}
