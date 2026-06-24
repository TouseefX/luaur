use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_interior_tests_are_cached() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let optional_number_ty = fixture.builtin_types.optionalNumberType;

    let table_a = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(number_ty)),
        ("Y", Property::rw_type_id(number_ty)),
    ]));
    let table_b = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(optional_number_ty)),
        ("Y", Property::rw_type_id(optional_number_ty)),
    ]));

    assert!(!fixture
        .is_subtype_type_id_type_id(table_a, table_b)
        .is_subtype());

    let cached_result = fixture
        .subtyping
        .peek_cache()
        .find(&(number_ty, optional_number_ty))
        .expect("interior primitive subtype test should be cached");
    assert!(cached_result.is_subtype());

    let cached_result = fixture
        .subtyping
        .peek_cache()
        .find(&(table_a, table_b))
        .expect("outer table subtype test should be cached");
    assert!(!cached_result.is_subtype());
}
