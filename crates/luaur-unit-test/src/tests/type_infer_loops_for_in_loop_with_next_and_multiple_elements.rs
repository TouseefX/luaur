//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:185:type_infer_loops_for_in_loop_with_next_and_multiple_elements`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_loops_for_in_loop_with_next_and_multiple_elements

#[cfg(test)]
#[test]
fn type_infer_loops_for_in_loop_with_next_and_multiple_elements() {
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
        local n
        local s
        for i, v in next, { "foo", "bar" } do
            n = i
            s = v
            print(i, v)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "number?",
            to_string_type_id(fixture.base.require_type_string(&String::from("n")))
        );
        assert_eq!(
            "string?",
            to_string_type_id(fixture.base.require_type_string(&String::from("s")))
        );
        assert_eq!(
            "number",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 6,
                column: 18
            }))
        );
        assert_eq!(
            "string",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 6,
                column: 21
            }))
        );
    } else {
        assert_eq!(
            "number",
            to_string_type_id(fixture.base.require_type_string(&String::from("n")))
        );
        assert_eq!(
            "string",
            to_string_type_id(fixture.base.require_type_string(&String::from("s")))
        );
    }
}
