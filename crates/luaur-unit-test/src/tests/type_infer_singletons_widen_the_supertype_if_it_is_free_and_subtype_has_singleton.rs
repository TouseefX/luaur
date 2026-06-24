//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:497:type_infer_singletons_widen_the_supertype_if_it_is_free_and_subtype_has_singleton`
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
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - translates_to -> rust_item type_infer_singletons_widen_the_supertype_if_it_is_free_and_subtype_has_singleton

#[cfg(test)]
#[test]
fn type_infer_singletons_widen_the_supertype_if_it_is_free_and_subtype_has_singleton() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo(f, x)
            if x == "hi" then
                f(x)
                f("foo")
            end
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
            column: 18
        }))
    );
    assert_eq!(
        "<a, b...>((string) -> (b...), a) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
