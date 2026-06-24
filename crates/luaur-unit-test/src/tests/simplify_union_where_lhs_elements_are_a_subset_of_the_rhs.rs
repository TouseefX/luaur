//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:218:simplify_union_where_lhs_elements_are_a_subset_of_the_rhs`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item simplify_union_where_lhs_elements_are_a_subset_of_the_rhs

#[cfg(test)]
#[test]
fn simplify_union_where_lhs_elements_are_a_subset_of_the_rhs() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;

    let lhs = fixture.union_(number_ty, string_ty);
    let rhs = fixture.union_(string_ty, number_ty);
    let actual = fixture.union_(lhs, rhs);

    assert_eq!(
        "number | string",
        to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
}
