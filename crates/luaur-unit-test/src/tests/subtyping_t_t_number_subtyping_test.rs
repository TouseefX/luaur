use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_t_t_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let generic_nothing_to_t_ty = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t]);
    let nothing_to_number_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![number_ty]);

    assert!(fixture
        .is_subtype_type_id_type_id(generic_nothing_to_t_ty, nothing_to_number_ty)
        .is_subtype());
}
