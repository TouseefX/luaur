//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:473:simplify_union`
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
//!   - translates_to -> rust_item simplify_union

#[cfg(test)]
#[test]
fn simplify_union() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let table_ty = fixture.table_ty;
    let truthy_ty = fixture.truthy_ty;
    let optional_string_ty = unsafe { (*fixture.base.builtin_types).optionalStringType };

    let t1 = fixture.arena.add_type(UnionType {
        options: vec![number_ty, string_ty, nil_ty, table_ty],
    });

    let actual = fixture.intersect(t1, nil_ty);
    assert_eq!(nil_ty, actual);

    let actual = fixture.intersect(optional_string_ty, truthy_ty);
    assert_eq!(string_ty, actual);
}
