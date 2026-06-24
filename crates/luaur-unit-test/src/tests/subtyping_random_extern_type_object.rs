use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_random_extern_type_object() {
    let mut fixture = SubtypeFixture::default();
    let extern_ty = fixture.builtin_types.externType;
    let object_ty = fixture.builtin_types.objectType;

    assert!(!fixture
        .is_subtype_type_id_type_id(extern_ty, object_ty)
        .is_subtype());
}
