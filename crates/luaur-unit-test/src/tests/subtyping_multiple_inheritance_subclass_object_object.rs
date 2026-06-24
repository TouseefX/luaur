use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_multiple_inheritance_subclass_object_object() {
    let mut fixture = SubtypeFixture::default();
    let b = fixture.obj("B", None);
    let a = fixture.obj("A", Some(b));
    let object_ty = fixture.builtin_types.objectType;

    assert!(fixture
        .is_subtype_type_id_type_id(a, object_ty)
        .is_subtype());
    assert!(fixture
        .is_subtype_type_id_type_id(b, object_ty)
        .is_subtype());
    assert!(!fixture.is_subtype_type_id_type_id(b, a).is_subtype());
}
