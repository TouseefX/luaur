use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_t_t_t_string_number() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let nothing_to_two_ts_ty =
        fixture.generic_fn(vec![generic_t], vec![], vec![generic_t, generic_t]);
    let nothing_to_string_and_number_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![],
            vec![string_ty, number_ty],
        );

    assert!(!fixture
        .is_subtype_type_id_type_id(nothing_to_two_ts_ty, nothing_to_string_and_number_ty)
        .is_subtype());
}
