use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_results_that_are_contingent_on_generics_are_not_cached() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let generic_t_to_t_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![generic_t]);
    let number_to_number_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty],
        vec![number_ty],
    );

    assert!(fixture
        .is_subtype_type_id_type_id(generic_t_to_t_ty, number_to_number_ty)
        .is_subtype());
    assert!(fixture.subtyping.peek_cache().empty());
}
