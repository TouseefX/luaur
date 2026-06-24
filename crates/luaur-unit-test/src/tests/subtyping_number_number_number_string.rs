use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

#[cfg(test)]
#[test]
fn subtyping_number_number_number_string() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let left = fixture.pack_initializer_list_type_id_type_pack_variant(
        vec![number_ty],
        TypePackVariant::Variadic(VariadicTypePack::new(number_ty)),
    );
    let right = fixture.pack_initializer_list_type_id_type_pack_variant(
        vec![number_ty],
        TypePackVariant::Variadic(VariadicTypePack::new(string_ty)),
    );

    assert!(!fixture
        .is_subtype_type_pack_id_type_pack_id(left, right)
        .is_subtype());
}
