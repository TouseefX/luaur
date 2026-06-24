use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_t_t() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let nothing_to_number_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![number_ty]);
    let generic_nothing_to_t_ty = fixture.generic_fn(vec![generic_t], vec![], vec![generic_t]);

    assert!(!fixture
        .is_subtype_type_id_type_id(nothing_to_number_ty, generic_nothing_to_t_ty)
        .is_subtype());
}
