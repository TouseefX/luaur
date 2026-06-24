use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_root_root() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let left = fixture.join(hierarchy.child_class, hierarchy.root_class);

    assert!(fixture
        .is_subtype_type_id_type_id(left, hierarchy.root_class)
        .is_subtype());
}
