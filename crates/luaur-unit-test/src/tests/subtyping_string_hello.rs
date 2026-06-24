use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_string_hello() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let hello_ty = fixture.str("hello");

    assert!(!fixture
        .is_subtype_type_id_type_id(string_ty, hello_ty)
        .is_subtype());
}
