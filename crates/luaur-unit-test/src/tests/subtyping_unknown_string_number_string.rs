use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_unknown_string_number_string() {
    let mut fixture = SubtypeFixture::default();
    let unknown_ty = fixture.builtin_types.unknownType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let unknown_to_string_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![unknown_ty],
        vec![string_ty],
    );
    let number_to_string_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![string_ty],
    );

    assert!(fixture
        .is_subtype_type_id_type_id(unknown_to_string_ty, number_to_string_ty)
        .is_subtype());
}
