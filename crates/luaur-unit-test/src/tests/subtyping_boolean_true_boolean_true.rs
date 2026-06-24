use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_boolean_true_boolean_true() {
    let mut fixture = SubtypeFixture::default();
    let true_ty = fixture.builtin_types.trueType;
    let boolean_ty = fixture.builtin_types.booleanType;
    let boolean_and_true_ty = fixture.meet(boolean_ty, true_ty);

    assert!(fixture
        .is_subtype_type_id_type_id(boolean_and_true_ty, boolean_and_true_ty)
        .is_subtype());
}
