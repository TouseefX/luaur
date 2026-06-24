//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:73:unifier_2_t_u`
//! Source: `tests/Unifier2.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Unifier2.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Unifier2.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Unifier2.test.cpp
//! - outgoing:
//!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
//!   - type_ref -> enum UnifyResult (Analysis/include/Luau/Unifier2.h)
//!   - translates_to -> rust_item unifier_2_t_u

#[cfg(test)]
#[test]
fn unifier2_t_u() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::unify_result::UnifyResult;

    let mut fixture = Unifier2Fixture::new();
    let (left, free_left) = fixture.fresh_type();
    let (right, free_right) = fixture.fresh_type();

    assert_eq!(UnifyResult::Ok, fixture.u2.unify(left, right));

    assert_eq!(
        "t1 where t1 = ('a <: (t1 <: 'b))",
        fixture.to_string_type_id(left)
    );
    assert_eq!(
        "t1 where t1 = (('a <: t1) <: 'b)",
        fixture.to_string_type_id(right)
    );

    assert_eq!(
        "never",
        fixture.to_string_type_id(unsafe { (*free_left).lower_bound })
    );
    assert_eq!(
        "t1 where t1 = (('a <: t1) <: 'b)",
        fixture.to_string_type_id(unsafe { (*free_left).upper_bound })
    );

    assert_eq!(
        "t1 where t1 = ('a <: (t1 <: 'b))",
        fixture.to_string_type_id(unsafe { (*free_right).lower_bound })
    );
    assert_eq!(
        "unknown",
        fixture.to_string_type_id(unsafe { (*free_right).upper_bound })
    );
}
