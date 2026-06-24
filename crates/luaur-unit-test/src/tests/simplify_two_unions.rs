//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:483:simplify_two_unions`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_two_unions

#[cfg(test)]
#[test]
fn simplify_two_unions() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::vec;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_common::DFInt;

    let _sfi = ScopedFastInt::new(&DFInt::LuauSimplificationComplexityLimit, 10);
    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let boolean_ty = fixture.boolean_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let table_ty = fixture.table_ty;
    let falsy_ty = fixture.falsy_ty;

    let t1 = fixture.arena.add_type(UnionType {
        options: vec![number_ty, boolean_ty, string_ty, nil_ty, table_ty],
    });

    let actual = fixture.intersect_str(t1, falsy_ty);
    assert_eq!("false?", actual);
}
