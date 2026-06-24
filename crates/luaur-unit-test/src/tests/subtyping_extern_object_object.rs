use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_extern_object_object() {
    let mut fixture = SubtypeFixture::default();
    let my_object = fixture.obj("MyObject", None);
    let object_ty = fixture.builtin_types.objectType;

    assert!(fixture
        .is_subtype_type_id_type_id(my_object, object_ty)
        .is_subtype());
}
