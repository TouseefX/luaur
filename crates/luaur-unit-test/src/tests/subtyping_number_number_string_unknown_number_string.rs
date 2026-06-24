use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_number_string_unknown_number_string() {
    let mut fixture = SubtypeFixture::default();
    let unknown_ty = fixture.builtin_types.unknownType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let number_number_to_string_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![number_ty, number_ty],
            vec![string_ty],
        );
    let unknown_number_to_string_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![unknown_ty, number_ty],
            vec![string_ty],
        );

    assert!(!fixture
        .is_subtype_type_id_type_id(number_number_to_string_ty, unknown_number_to_string_ty)
        .is_subtype());
}
