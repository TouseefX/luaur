//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:571:simplify_simplify_stops_at_cycles`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_simplify_stops_at_cycles

#[cfg(test)]
#[test]
fn simplify_simplify_stops_at_cycles() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let any_ty = fixture.any_ty;

    let t = fixture.mk_table(&[]);
    let tt = unsafe { get_mutable_type_id::<TableType>(t).as_mut() }.expect("expected table");

    let t2 = fixture.mk_table(&[]);
    let t2t = unsafe { get_mutable_type_id::<TableType>(t2).as_mut() }.expect("expected table");

    tt.props
        .insert("cyclic".to_string(), Property::rw_type_id(t2));
    t2t.props
        .insert("cyclic".to_string(), Property::rw_type_id(t));

    let actual = fixture.intersect(t, unknown_ty);
    assert_eq!(t, actual);

    let actual = fixture.intersect(unknown_ty, t);
    assert_eq!(t, actual);

    let actual = fixture.intersect(t2, unknown_ty);
    assert_eq!(t2, actual);

    let actual = fixture.intersect(unknown_ty, t2);
    assert_eq!(t2, actual);

    let expected = "*error-type* | t1 where t1 = { cyclic: { cyclic: t1 } }";

    let actual = fixture.intersect_str(t, any_ty);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(any_ty, t);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(t2, any_ty);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(any_ty, t2);
    assert_eq!(expected, actual);
}
