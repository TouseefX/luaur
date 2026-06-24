//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:238:type_infer_singletons_enums_using_singletons_subtyping`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_enums_using_singletons_subtyping

#[cfg(test)]
#[test]
fn type_infer_singletons_enums_using_singletons_subtyping() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyEnum1 = "foo" | "bar"
        type MyEnum2 = MyEnum1 | "baz"
        local a : MyEnum1 = "foo"
        local b : MyEnum2 = a
        local c : string = b
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
