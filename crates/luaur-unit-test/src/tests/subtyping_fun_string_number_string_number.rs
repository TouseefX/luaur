use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_fun_string_number_string_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;
    let function_ty = fixture.builtin_types.functionType;
    let number_to_string_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![string_ty],
    );

    let not_function_ty = fixture.negate(function_ty);
    let sub_ty = fixture.meet(not_function_ty, number_to_string_ty);

    assert!(fixture
        .is_subtype_type_id_type_id(sub_ty, number_to_string_ty)
        .is_subtype());
}
