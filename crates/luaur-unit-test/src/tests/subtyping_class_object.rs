use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_class_object() {
    let mut fixture = SubtypeFixture::default();
    let class_ty = fixture.builtin_types.classType;
    let object_ty = fixture.builtin_types.objectType;

    assert!(!fixture
        .is_subtype_type_id_type_id(class_ty, object_ty)
        .is_subtype());
}
