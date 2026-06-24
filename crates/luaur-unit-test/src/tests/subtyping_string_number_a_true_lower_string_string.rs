use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_string_number_a_true_lower_string_string() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let number_ty = fixture.builtin_types.numberType;
    let true_ty = fixture.builtin_types.trueType;
    let a_ty = fixture.str("a");

    let string_or_number = fixture.join(string_ty, number_ty);
    let a_or_true = fixture.join(a_ty, true_ty);
    let sub_ty = fixture.meet(string_or_number, a_or_true);
    let table_with_lower = fixture.table_with_lower();

    assert!(fixture
        .is_subtype_type_id_type_id(sub_ty, table_with_lower)
        .is_subtype());
}
