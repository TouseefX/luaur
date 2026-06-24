//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:324:simplify_functions`
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
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_functions

#[cfg(test)]
#[test]
fn simplify_functions() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let fn1_ty = fixture.fn1_ty;
    let fn2_ty = fixture.fn2_ty;
    let function_ty = fixture.function_ty;

    let actual = fixture.intersect(fn1_ty, function_ty);
    assert_eq!(fn1_ty, actual);
    let actual = fixture.intersect(function_ty, fn1_ty);
    assert_eq!(fn1_ty, actual);

    let actual = fixture.intersect_str(fn1_ty, fn2_ty);
    assert_eq!("(() -> ()) & ((...any) -> ())", actual);
}
