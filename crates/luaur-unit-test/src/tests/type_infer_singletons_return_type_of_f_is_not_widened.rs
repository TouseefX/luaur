//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:517:type_infer_singletons_return_type_of_f_is_not_widened`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_return_type_of_f_is_not_widened

#[cfg(test)]
#[test]
fn type_infer_singletons_return_type_of_f_is_not_widened() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo(f, x): "hello"? -- anyone there?
            return if x == "hi"
                then f(x)
                else nil
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        r#""hi""#,
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 3,
            column: 23
        }))
    );
    assert_eq!(
        r#"<a, b, c...>((string) -> (a, c...), b) -> "hello"?"#,
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
