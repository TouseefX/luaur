//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:316:simplify_primitives_and_singletons`
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
//!   - translates_to -> rust_item simplify_primitives_and_singletons

#[cfg(test)]
#[test]
fn simplify_primitives_and_singletons() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let hello_ty = fixture.hello_ty;
    let string_ty = fixture.string_ty;
    let world_ty = fixture.world_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(hello_ty, string_ty);
    assert_eq!(hello_ty, actual);
    let actual = fixture.intersect(string_ty, hello_ty);
    assert_eq!(hello_ty, actual);

    let actual = fixture.intersect(world_ty, hello_ty);
    assert_eq!(never_ty, actual);
}
