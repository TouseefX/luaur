use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_class_a_and_b_not_subtypes_of_each_other() {
    let mut fixture = SubtypeFixture::default();
    let a = fixture.user_defined_cls("A", None);
    let b = fixture.user_defined_cls("B", None);

    assert!(!fixture.is_subtype_type_id_type_id(a, b).is_subtype());
    assert!(!fixture.is_subtype_type_id_type_id(b, a).is_subtype());
}
