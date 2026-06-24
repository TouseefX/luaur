use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_false_boolean_true() {
    let mut fixture = SubtypeFixture::default();
    let false_ty = fixture.builtin_types.falseType;
    let true_ty = fixture.builtin_types.trueType;
    let boolean_ty = fixture.builtin_types.booleanType;
    let boolean_and_true_ty = fixture.meet(boolean_ty, true_ty);

    assert!(!fixture
        .is_subtype_type_id_type_id(false_ty, boolean_and_true_ty)
        .is_subtype());
}
