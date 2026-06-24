use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_number_unknown() {
    let mut fixture = SubtypeFixture::default();
    let optional_number_ty = fixture.builtin_types.optionalNumberType;
    let unknown_ty = fixture.builtin_types.unknownType;

    assert!(fixture
        .is_subtype_type_id_type_id(optional_number_ty, unknown_ty)
        .is_subtype());
}
