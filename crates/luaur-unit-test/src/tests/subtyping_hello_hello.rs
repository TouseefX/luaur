use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_hello_hello() {
    let mut fixture = SubtypeFixture::default();
    let hello_ty = fixture.str("hello");
    let hello_ty_2 = fixture.str("hello");

    assert!(fixture
        .is_subtype_type_id_type_id(hello_ty, hello_ty_2)
        .is_subtype());
}
