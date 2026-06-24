use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_another_child_child_another_child() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let left = fixture.join(hierarchy.child_class, hierarchy.another_child_class);
    let right = fixture.join(hierarchy.child_class, hierarchy.another_child_class);

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
