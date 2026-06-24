//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:688:simplify_relate_write_only_number_with_number`
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
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_relate_write_only_number_with_number

#[cfg(test)]
#[test]
fn simplify_relate_write_only_number_with_number() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let nil_ty = fixture.nil_ty;

    let left_ty = fixture.mk_table_props(&[("x", Property::writeonly(number_ty))]);
    let optional_number = fixture.arena.add_type(UnionType {
        options: vec![nil_ty, number_ty],
    });
    let right_ty = fixture.mk_table_props(&[("x", Property::rw_type_id(optional_number))]);

    let actual = fixture.intersect_str(left_ty, right_ty);
    assert_eq!("{ write x: number } & { x: number? }", actual);
    let actual = fixture.intersect_str(right_ty, left_ty);
    assert_eq!("{ write x: number } & { x: number? }", actual);

    let actual = fixture.union_(left_ty, right_ty);
    assert_eq!(
        "{ write x: number } | { x: number? }",
        luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
            actual,
            &mut fixture.opts
        )
    );
    let actual = fixture.union_(right_ty, left_ty);
    assert_eq!(
        "{ write x: number } | { x: number? }",
        luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options(
            actual,
            &mut fixture.opts
        )
    );
}
