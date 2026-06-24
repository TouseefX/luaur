use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_string_hello_hello() {
    let mut fixture = SubtypeFixture::default();
    let string_ty = fixture.builtin_types.stringType;
    let hello_ty = fixture.str("hello");
    let hello_or_hello_ty = fixture.join(hello_ty, hello_ty);

    assert!(!fixture
        .is_subtype_type_id_type_id(string_ty, hello_or_hello_ty)
        .is_subtype());
}
