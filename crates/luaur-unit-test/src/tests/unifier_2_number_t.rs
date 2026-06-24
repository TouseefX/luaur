//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:63:unifier_2_number_t`
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
//!   - translates_to -> rust_item unifier_2_number_t

#[cfg(test)]
#[test]
fn unifier2_number_t() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::unify_result::UnifyResult;

    let mut fixture = Unifier2Fixture::new();
    let (right, free_right) = fixture.fresh_type();

    assert_eq!(
        UnifyResult::Ok,
        fixture.u2.unify(fixture.builtin_types.numberType, right)
    );

    assert_eq!(
        "number",
        fixture.to_string_type_id(unsafe { (*free_right).lower_bound })
    );
    assert_eq!(
        "unknown",
        fixture.to_string_type_id(unsafe { (*free_right).upper_bound })
    );
}
