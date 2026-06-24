use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_another_child_number() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let number_ty = fixture.builtin_types.numberType;
    let left = fixture.meet(hierarchy.child_class, hierarchy.another_child_class);

    assert!(fixture
        .is_subtype_type_id_type_id(left, number_ty)
        .is_subtype());
}
