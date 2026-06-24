use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_extern_class_class() {
    let mut fixture = SubtypeFixture::default();
    let my_class = fixture.user_defined_cls("MyClass", None);
    let class_ty = fixture.builtin_types.classType;

    assert!(fixture
        .is_subtype_type_id_type_id(my_class, class_ty)
        .is_subtype());
}
