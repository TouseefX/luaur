use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_object_class() {
    let mut fixture = SubtypeFixture::default();
    let object_ty = fixture.builtin_types.objectType;
    let class_ty = fixture.builtin_types.classType;

    assert!(!fixture
        .is_subtype_type_id_type_id(object_ty, class_ty)
        .is_subtype());
}
