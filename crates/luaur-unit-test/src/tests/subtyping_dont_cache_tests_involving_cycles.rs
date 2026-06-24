use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
use luaur_analysis::records::blocked_type::BlockedType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::type_aliases::type_variant::TypeVariant;

#[cfg(test)]
#[test]
fn subtyping_dont_cache_tests_involving_cycles() {
    let mut fixture = SubtypeFixture::default();

    let table_a = fixture.arena.add_type(BlockedType::default());
    let table_a_2 = fixture.tbl(SubtypeFixture::props(vec![(
        "self",
        Property::rw_type_id(table_a),
    )]));
    unsafe {
        (*as_mutable_type_id(table_a)).ty = TypeVariant::Bound(table_a_2);
    }

    let table_b = fixture.arena.add_type(BlockedType::default());
    let table_b_2 = fixture.tbl(SubtypeFixture::props(vec![(
        "self",
        Property::rw_type_id(table_b),
    )]));
    unsafe {
        (*as_mutable_type_id(table_b)).ty = TypeVariant::Bound(table_b_2);
    }

    assert!(fixture
        .is_subtype_type_id_type_id(table_a, table_b)
        .is_subtype());
    assert!(fixture
        .subtyping
        .peek_cache()
        .find(&(table_a, table_b))
        .is_none());
}
