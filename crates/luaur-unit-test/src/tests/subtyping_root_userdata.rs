use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_root_userdata() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let extern_ty = fixture.builtin_types.externType;

    assert!(fixture
        .is_subtype_type_id_type_id(hierarchy.root_class, extern_ty)
        .is_subtype());
}
