use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::function_type::FunctionType;

#[cfg(test)]
#[test]
fn subtyping_any_t_t() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let any_type_pack = fixture.builtin_types.anyTypePack;
    let empty_type_pack = fixture.builtin_types.emptyTypePack;

    let anys_to_nothing = fixture.arena.add_type(FunctionType::function_type_new(
        any_type_pack,
        empty_type_pack,
        None,
        false,
    ));
    let generic_t_to_anys = fixture.arena.add_type(FunctionType::function_type_new(
        generic_as,
        empty_type_pack,
        None,
        false,
    ));

    assert!(fixture
        .is_subtype_type_id_type_id(anys_to_nothing, generic_t_to_anys)
        .is_subtype());
}
