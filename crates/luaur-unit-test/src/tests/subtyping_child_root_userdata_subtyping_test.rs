use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_root_userdata() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let extern_ty = fixture.builtin_types.externType;
    let left = fixture.meet(hierarchy.child_class, hierarchy.root_class);

    assert!(fixture
        .is_subtype_type_id_type_id(left, extern_ty)
        .is_subtype());
}
