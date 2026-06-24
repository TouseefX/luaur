//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.negations.test.cpp:69:type_infer_negations_compare_cofinite_strings`
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
//!   - translates_to -> rust_item type_infer_negations_compare_cofinite_strings

#[cfg(test)]
#[test]
fn type_infer_negations_compare_cofinite_strings() {
    use crate::records::negation_fixture::NegationFixture;

    let mut fixture = NegationFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local u : Not<"a">
local v : "b"
if u == v then
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
