use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_t_t_u_u() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");

    let generic_t_to_nothing_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let generic_u_to_nothing_ty = fixture.generic_fn(vec![generic_u], vec![generic_u], vec![]);

    assert!(fixture
        .is_subtype_type_id_type_id(generic_t_to_nothing_ty, generic_u_to_nothing_ty)
        .is_subtype());
}
