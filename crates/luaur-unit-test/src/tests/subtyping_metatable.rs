use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_metatable() {
    let mut fixture = SubtypeFixture::default();
    let left = fixture.meta(SubtypeFixture::props(vec![]), SubtypeFixture::props(vec![]));
    let right = fixture.tbl(SubtypeFixture::props(vec![]));

    assert!(fixture.is_subtype_type_id_type_id(left, right).is_subtype());
}
