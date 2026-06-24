use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_hello_string() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let string_ty = fixture.builtin_types.stringType;

    assert!(fixture
        .is_subtype_type_id_type_id(hello_ty, string_ty)
        .is_subtype());
}
