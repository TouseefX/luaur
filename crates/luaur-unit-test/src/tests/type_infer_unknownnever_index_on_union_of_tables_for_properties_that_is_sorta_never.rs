//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:286:type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never

#[cfg(test)]
#[test]
fn type_infer_unknownnever_index_on_union_of_tables_for_properties_that_is_sorta_never() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Disjoint = {foo: string, bar: unknown, tag: "ok"} | {foo: never, baz: unknown, tag: "err"}

        function f(disjoint: Disjoint)
            return disjoint.foo
        end

        local foo = f({foo = 5 :: never, bar = true, tag = "ok"})
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
