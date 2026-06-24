//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:649:simplify_x_number_y_number_read_x_unknown`
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
//!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_x_number_y_number_read_x_unknown

#[cfg(test)]
#[test]
fn simplify_x_number_y_number_read_x_unknown() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::records::property_type::Property;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let unknown_ty = fixture.unknown_ty;

    let left_ty = fixture.mk_table(&[("x", number_ty), ("y", number_ty)]);
    let right_ty = fixture.mk_table_props(&[("x", Property::readonly(unknown_ty))]);

    let actual = fixture.intersect(left_ty, right_ty);
    assert_eq!(left_ty, actual);
}
