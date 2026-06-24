use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_true_boolean() {
    let mut fixture = SubtypeFixture::default();
    let true_ty = fixture.builtin_types.trueType;
    let boolean_ty = fixture.builtin_types.booleanType;

    assert!(fixture
        .is_subtype_type_id_type_id(true_ty, boolean_ty)
        .is_subtype());
}
