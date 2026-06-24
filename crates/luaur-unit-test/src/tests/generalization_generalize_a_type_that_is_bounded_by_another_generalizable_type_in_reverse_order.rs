//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:88:generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order

#[cfg(test)]
#[test]
fn generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order()
{
    use crate::records::generalization_fixture::GeneralizationFixture;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = GeneralizationFixture::new();
    let (t1, ft1) = fixture.fresh_type();
    let (t2, ft2) = fixture.fresh_type();

    unsafe {
        (*ft1).lower_bound = t2;
        (*ft2).upper_bound = t1;
        (*ft2).lower_bound = fixture.builtin_types.unknownType;
    }

    let t1_generalized = fixture.generalize(t1);
    assert!(t1_generalized.is_some());

    assert_eq!(unsafe { follow_type_id(t1) }, unsafe { follow_type_id(t2) });

    let t2_generalized = fixture.generalize(t2);
    assert!(t2_generalized.is_some());

    assert_eq!(fixture.builtin_types.unknownType, unsafe {
        follow_type_id(t1)
    });
    assert_eq!(fixture.builtin_types.unknownType, unsafe {
        follow_type_id(t2)
    });
}
