use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_class_a_and_b_class_subtypes() {
    let mut fixture = SubtypeFixture::default();
    let a = fixture.user_defined_cls("A", None);
    let b = fixture.user_defined_cls("B", None);
    let class_ty = fixture.builtin_types.classType;

    assert!(fixture.is_subtype_type_id_type_id(a, class_ty).is_subtype());
    assert!(fixture.is_subtype_type_id_type_id(b, class_ty).is_subtype());
}
