use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::property_type::Property;

#[cfg(test)]
#[test]
fn subtyping_x_t_t_x_u_u() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");
    let generic_t_to_nothing_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);
    let generic_u_to_nothing_ty = fixture.generic_fn(vec![generic_u], vec![generic_u], vec![]);
    let left = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::rw_type_id(generic_t_to_nothing_ty),
    )]));
    let right = fixture.tbl(SubtypeFixture::props(vec![(
        "x",
        Property::rw_type_id(generic_u_to_nothing_ty),
    )]));

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
