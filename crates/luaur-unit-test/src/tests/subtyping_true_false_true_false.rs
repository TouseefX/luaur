use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_true_false_true_false() {
    let mut fixture = SubtypeFixture::default();
    let true_ty = fixture.builtin_types.trueType;
    let false_ty = fixture.builtin_types.falseType;
    let true_or_false_ty = fixture.join(true_ty, false_ty);

    assert!(fixture
        .is_subtype_type_id_type_id(true_or_false_ty, true_or_false_ty)
        .is_subtype());
}
