use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;

#[cfg(test)]
#[test]
fn subtyping_unknown_t_t() {
    let mut fixture = SubtypeFixture::default();
    let generic_as = fixture.generic_pack("A");
    let unknown_ty = fixture.builtin_types.unknownType;
    let empty_type_pack = fixture.builtin_types.emptyTypePack;

    let unknown_type_pack = fixture
        .arena
        .add_type_pack_t(VariadicTypePack::new(unknown_ty));
    let unknowns_to_nothing = fixture.arena.add_type(FunctionType::function_type_new(
        unknown_type_pack,
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
        .is_subtype_type_id_type_id(unknowns_to_nothing, generic_t_to_anys)
        .is_subtype());
}
