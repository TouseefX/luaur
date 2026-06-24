use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_number_t_t_t() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let number_to_number_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![number_ty],
    );
    let generic_t_to_t_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![generic_t]);

    let f1 = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_to_number_ty],
        vec![],
    );
    let f2 = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(vec![generic_t_to_t_ty], vec![]);

    assert!(fixture.is_subtype_type_id_type_id(f1, f2).is_subtype());
}
