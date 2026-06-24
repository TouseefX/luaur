use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_string_unknown_number_string_string() {
    let mut fixture = SubtypeFixture::default();
    let unknown_ty = fixture.builtin_types.unknownType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let number_to_string_and_unknown_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![number_ty],
            vec![string_ty, unknown_ty],
        );
    let number_to_two_strings_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![number_ty],
            vec![string_ty, string_ty],
        );

    assert!(!fixture
        .is_subtype_type_id_type_id(number_to_string_and_unknown_ty, number_to_two_strings_ty)
        .is_subtype());
}
