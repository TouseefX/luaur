use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;

    assert!(fixture
        .is_subtype_type_id_type_id(number_ty, number_ty)
        .is_subtype());
}
