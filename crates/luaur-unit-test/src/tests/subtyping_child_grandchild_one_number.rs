use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_grandchild_one_number() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let number_ty = fixture.builtin_types.numberType;
    let not_grandchild_one = fixture.negate(hierarchy.grandchild_one_class);
    let left = fixture.meet(hierarchy.child_class, not_grandchild_one);

    assert!(!fixture
        .is_subtype_type_id_type_id(left, number_ty)
        .is_subtype());
}
