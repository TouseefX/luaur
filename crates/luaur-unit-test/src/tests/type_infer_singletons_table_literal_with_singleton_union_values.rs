//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:689:type_infer_singletons_table_literal_with_singleton_union_values`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_table_literal_with_singleton_union_values

#[cfg(test)]
#[test]
fn type_infer_singletons_table_literal_with_singleton_union_values() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t1: {[string]: "a" | "b"} = { a = "a", b = "b" }
        local t2: {[string]: "a" | true} = { a = "a", b = true }
        local t3: {[string]: "a" | nil} = { a = "a" }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
