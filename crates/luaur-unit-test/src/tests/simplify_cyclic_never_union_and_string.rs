//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:612:simplify_cyclic_never_union_and_string`
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
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_cyclic_never_union_and_string

#[cfg(test)]
#[test]
fn simplify_cyclic_never_union_and_string() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let never_ty = fixture.never_ty;
    let string_ty = fixture.string_ty;

    let left_type = fixture.arena.add_type(UnionType {
        options: vec![never_ty, never_ty],
    });
    let left_union = unsafe { get_mutable_type_id::<UnionType>(left_type).as_mut() }
        .expect("expected mutable union");
    left_union.options[0] = left_type;

    let actual = fixture.union_(left_type, string_ty);
    assert_eq!(string_ty, actual);
}
