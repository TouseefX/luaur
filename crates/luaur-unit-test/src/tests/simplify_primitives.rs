//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:283:simplify_primitives`
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
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_primitives

#[cfg(test)]
#[test]
fn simplify_primitives() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::records::primitive_type::PrimitiveType;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;
    let function_ty = fixture.function_ty;
    let table_ty = fixture.table_ty;
    let any_ty = fixture.any_ty;
    let nil_ty = fixture.nil_ty;

    let number_ty_duplicate = fixture.arena.add_type(PrimitiveType {
        r#type: PrimitiveType::Number,
        metatable: None,
    });

    let actual = fixture.intersect(number_ty, number_ty_duplicate);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(number_ty, string_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, function_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(function_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, table_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(table_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect_str(any_ty, number_ty);
    assert_eq!("*error-type* | number", actual);
    let actual = fixture.intersect_str(number_ty, any_ty);
    assert_eq!("*error-type* | number", actual);

    let actual = fixture.intersect(string_ty, nil_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, string_ty);
    assert_eq!(never_ty, actual);
}
