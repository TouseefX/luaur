//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:174:simplify_boolean_and_truthy_and_falsy`
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
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_boolean_and_truthy_and_falsy

#[cfg(test)]
#[test]
fn simplify_boolean_and_truthy_and_falsy() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let boolean_ty = fixture.boolean_ty;
    let nil_ty = fixture.nil_ty;
    let truthy_ty = fixture.truthy_ty;
    let true_ty = fixture.true_ty;
    let optional_boolean_ty = fixture.arena.add_type(UnionType {
        options: vec![boolean_ty, nil_ty],
    });

    let actual = fixture.intersect(boolean_ty, truthy_ty);
    assert_eq!(true_ty, actual);
    let actual = fixture.intersect(optional_boolean_ty, truthy_ty);
    assert_eq!(true_ty, actual);
}
