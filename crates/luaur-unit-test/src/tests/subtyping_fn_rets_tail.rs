use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_rets::PathBuilderRets;
use luaur_analysis::methods::path_builder_tail::PathBuilderTail;
use luaur_analysis::methods::path_builder_variadic::PathBuilderVariadic;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

#[cfg(test)]
#[test]
fn subtyping_fn_rets_tail() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let sub_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id_type_pack_variant(
            vec![],
            vec![],
            TypePackVariant::Variadic(VariadicTypePack::new(number_ty)),
        );
    let super_ty = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id_type_pack_variant(
            vec![],
            vec![],
            TypePackVariant::Variadic(VariadicTypePack::new(string_ty)),
        );

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());
    assert_eq!(1, result.reasoning().size());

    let mut sub_path = PathBuilder::new();
    let mut super_path = PathBuilder::new();
    let expected = SubtypingReasoning::new(
        sub_path.rets().tail().variadic().build(),
        super_path.rets().tail().variadic().build(),
        SubtypingVariance::Covariant,
    );
    assert!(result.reasoning().contains(&expected));
}
