use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_semantic_subtyping_disj() {
    let mut fixture = SubtypeFixture::default();
    let unknown_ty = fixture.builtin_types.unknownType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;
    let not_number = fixture.negate(number_ty);
    let not_string = fixture.negate(string_ty);
    let super_ty = fixture.join(not_number, not_string);
    let result = fixture.is_subtype_type_id_type_id(unknown_ty, super_ty);

    assert!(result.is_subtype());
}
