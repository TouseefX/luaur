//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:787:subtyping_a_a_number_number_number`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

#[cfg(test)]
#[test]
fn subtyping_a_a_number_number_number() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_as = fixture.generic_pack("A");
    let number_pack = fixture.pack_initializer_list_type_id(vec![number_ty]);

    let generic_as_to_number_ty =
        fixture.generic_pack_fn(vec![generic_as], generic_as, number_pack);
    let numbers_to_number_ty = fixture
        .fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id(
            vec![],
            TypePackVariant::Variadic(VariadicTypePack::new(number_ty)),
            vec![number_ty],
        );

    assert!(fixture
        .is_subtype_type_id_type_id(generic_as_to_number_ty, numbers_to_number_ty)
        .is_subtype());
}
