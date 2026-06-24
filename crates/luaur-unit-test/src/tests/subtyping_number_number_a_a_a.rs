use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_number_a_a_a() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_as = fixture.generic_pack("A");

    let number_to_number_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![number_ty],
    );
    let generic_as_to_as_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);

    assert!(!fixture
        .is_subtype_type_id_type_id(number_to_number_ty, generic_as_to_as_ty)
        .is_subtype());
}
