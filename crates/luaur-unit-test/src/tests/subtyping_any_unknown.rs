use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_any_unknown() {
    let mut fixture = SubtypeFixture::default();
    let any_ty = fixture.builtin_types.anyType;
    let unknown_ty = fixture.builtin_types.unknownType;

    assert!(fixture
        .is_subtype_type_id_type_id(any_ty, unknown_ty)
        .is_subtype());
}
