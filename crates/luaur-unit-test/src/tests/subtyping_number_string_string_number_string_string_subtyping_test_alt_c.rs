//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:681:subtyping_number_string_string_number_string_string`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

#[cfg(test)]
#[test]
fn subtyping_number_string_string_number_string_string() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let number_and_strings_to_string_ty = fixture
        .fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id(
            vec![number_ty],
            TypePackVariant::Variadic(VariadicTypePack::new(string_ty)),
            vec![string_ty],
        );
    let number_and_string_to_string_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(
            vec![number_ty, string_ty],
            vec![string_ty],
        );

    assert!(fixture
        .is_subtype_type_id_type_id(
            number_and_strings_to_string_ty,
            number_and_string_to_string_ty,
        )
        .is_subtype());
}
