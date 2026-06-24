//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:148:simplify_nil`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_nil

#[cfg(test)]
#[test]
fn simplify_nil() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let nil_ty = fixture.nil_ty;
    let never_ty = fixture.never_ty;
    let number_ty = fixture.number_ty;
    let true_ty = fixture.true_ty;
    let table_ty = fixture.table_ty;

    let actual = fixture.intersect(nil_ty, nil_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(nil_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, true_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, table_ty);
    assert_eq!(never_ty, actual);
}
