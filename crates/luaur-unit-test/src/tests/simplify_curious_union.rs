//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:491:simplify_curious_union`
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
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_curious_union

#[cfg(test)]
#[test]
fn simplify_curious_union() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let false_ty = fixture.false_ty;
    let nil_ty = fixture.nil_ty;
    let number_ty = fixture.number_ty;

    let false_intersection = fixture.arena.add_type(IntersectionType {
        parts: vec![free_ty, false_ty],
    });
    let nil_intersection = fixture.arena.add_type(IntersectionType {
        parts: vec![free_ty, nil_ty],
    });
    let curious = fixture.arena.add_type(UnionType {
        options: vec![false_intersection, nil_intersection],
    });

    let actual = fixture.union_(curious, number_ty);
    assert_eq!(
        "('a & false) | ('a & nil) | number",
        to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
}
