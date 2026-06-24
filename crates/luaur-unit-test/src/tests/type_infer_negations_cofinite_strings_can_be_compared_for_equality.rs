//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:51:type_infer_negations_cofinite_strings_can_be_compared_for_equality`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_negations_cofinite_strings_can_be_compared_for_equality

#[cfg(test)]
#[test]
fn type_infer_negations_cofinite_strings_can_be_compared_for_equality() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(e)
            if e == 'strictEqual' then
                e = 'strictEqualObject'
            end
            if e == 'deepStrictEqual' or e == 'strictEqual' then
            elseif e == 'notDeepStrictEqual' or e == 'notStrictEqual' then
            end
            return e
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(string) -> string",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
