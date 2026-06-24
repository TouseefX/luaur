//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:183:simplify_any_and_indeterminate_types`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_any_and_indeterminate_types

#[cfg(test)]
#[test]
fn simplify_any_and_indeterminate_types() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;
    let error_ty = fixture.error_ty;

    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);

    let actual = fixture.intersect_str(any_ty, generic_ty);
    assert_eq!("*error-type* | b", actual);
    let actual = fixture.intersect_str(generic_ty, any_ty);
    assert_eq!("*error-type* | b", actual);

    let actual = fixture.intersect(any_ty, blocked_ty);
    let any_rhs_blocked = unsafe {
        get_type_id::<UnionType>(actual)
            .as_ref()
            .expect("expected union")
    };
    assert_eq!(2, any_rhs_blocked.options.len());
    assert_eq!(blocked_ty, any_rhs_blocked.options[0]);
    assert_eq!(error_ty, any_rhs_blocked.options[1]);

    let actual = fixture.intersect(blocked_ty, any_ty);
    let any_lhs_blocked = unsafe {
        get_type_id::<UnionType>(actual)
            .as_ref()
            .expect("expected union")
    };
    assert_eq!(2, any_lhs_blocked.options.len());
    assert_eq!(blocked_ty, any_lhs_blocked.options[0]);
    assert_eq!(error_ty, any_lhs_blocked.options[1]);

    let actual = fixture.intersect(any_ty, pending_ty);
    let any_rhs_pending = unsafe {
        get_type_id::<UnionType>(actual)
            .as_ref()
            .expect("expected union")
    };
    assert_eq!(2, any_rhs_pending.options.len());
    assert_eq!(pending_ty, any_rhs_pending.options[0]);
    assert_eq!(error_ty, any_rhs_pending.options[1]);

    let actual = fixture.intersect(pending_ty, any_ty);
    let any_lhs_pending = unsafe {
        get_type_id::<UnionType>(actual)
            .as_ref()
            .expect("expected union")
    };
    assert_eq!(2, any_lhs_pending.options.len());
    assert_eq!(pending_ty, any_lhs_pending.options[0]);
    assert_eq!(error_ty, any_lhs_pending.options[1]);
}
