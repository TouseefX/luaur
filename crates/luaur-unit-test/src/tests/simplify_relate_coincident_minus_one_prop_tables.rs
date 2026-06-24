//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:727:simplify_relate_coincident_minus_one_prop_tables`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_relate_coincident_minus_one_prop_tables

#[cfg(test)]
#[test]
fn simplify_relate_coincident_minus_one_prop_tables() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::property_type::Property;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let boolean_ty = fixture.boolean_ty;
    let string_ty = fixture.string_ty;

    let left_ty = fixture.mk_table_props(&[
        ("x", Property::rw_type_id(number_ty)),
        ("y", Property::rw_type_id(boolean_ty)),
    ]);
    let right_ty = fixture.mk_table_props(&[
        ("x", Property::rw_type_id(number_ty)),
        ("y", Property::rw_type_id(boolean_ty)),
        ("z", Property::rw_type_id(string_ty)),
    ]);

    let actual = fixture.intersect_str(left_ty, right_ty);
    assert_eq!(
        "{ x: number, y: boolean } & { x: number, y: boolean, z: string }",
        actual
    );
    let actual = fixture.intersect_str(right_ty, left_ty);
    assert_eq!(
        "{ x: number, y: boolean } & { x: number, y: boolean, z: string }",
        actual
    );

    let actual = fixture.union_(left_ty, right_ty);
    assert_eq!(
        "{ x: number, y: boolean } | { x: number, y: boolean, z: string }",
        to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
    let actual = fixture.union_(right_ty, left_ty);
    assert_eq!(
        "{ x: number, y: boolean } | { x: number, y: boolean, z: string }",
        to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
}
