use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_b_c_b_c_a_a_a() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let generic_bs = fixture.generic_pack("B");
    let generic_cs = fixture.generic_pack("C");

    let generic_bs_to_cs_ty =
        fixture.generic_pack_fn(vec![generic_bs, generic_cs], generic_bs, generic_cs);
    let generic_as_to_as_ty = fixture.generic_pack_fn(vec![generic_as], generic_as, generic_as);

    assert!(fixture
        .is_subtype_type_id_type_id(generic_bs_to_cs_ty, generic_as_to_as_ty)
        .is_subtype());
}
