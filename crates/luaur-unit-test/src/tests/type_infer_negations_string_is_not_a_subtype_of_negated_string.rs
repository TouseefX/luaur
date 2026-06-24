//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:40:type_infer_negations_string_is_not_a_subtype_of_negated_string`
//! Source: `tests/TypeInfer.negations.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.negations.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.negations.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_negations_string_is_not_a_subtype_of_negated_string

#[cfg(test)]
#[test]
fn type_infer_negations_string_is_not_a_subtype_of_negated_string() {
    use crate::records::negation_fixture::NegationFixture;

    let mut fixture = NegationFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(arg: string & Not<"hello">) end
        local a: string
        foo(a)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
