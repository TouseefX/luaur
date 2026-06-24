use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::methods::path_builder_rets::PathBuilderRets;
use luaur_analysis::methods::path_builder_tail::PathBuilderTail;
use luaur_analysis::methods::path_builder_variadic::PathBuilderVariadic;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;
use luaur_analysis::records::type_pack::TypePack;

#[cfg(test)]
#[test]
fn subtyping_variadic_subpath_in_pack() {
    let mut fixture = SubtypeFixture::default();
    let any_type_pack = fixture.builtin_types.anyTypePack;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let sub_t_args = fixture.arena.add_type_pack_t(TypePack::new(
        vec![string_ty, string_ty],
        Some(any_type_pack),
    ));
    let super_t_args = fixture
        .arena
        .add_type_pack_t(TypePack::new(vec![number_ty], Some(any_type_pack)));

    let sub_ret_pack = fixture.pack_initializer_list_type_id(vec![number_ty]);
    let function_sub = fixture.arena.add_type(FunctionType::function_type_new(
        sub_t_args,
        sub_ret_pack,
        None,
        false,
    ));
    let super_ret_pack = fixture.pack_initializer_list_type_id(vec![string_ty]);
    let function_super = fixture.arena.add_type(FunctionType::function_type_new(
        super_t_args,
        super_ret_pack,
        None,
        false,
    ));

    let result = fixture.is_subtype_type_id_type_id(function_sub, function_super);

    let mut sub_rets = PathBuilder::new();
    let mut super_rets = PathBuilder::new();
    let mut sub_arg_0 = PathBuilder::new();
    let mut super_arg_0 = PathBuilder::new();
    let mut sub_arg_1 = PathBuilder::new();
    let mut super_arg_variadic = PathBuilder::new();
    let expected = [
        SubtypingReasoning::new(
            sub_rets.rets().index(0).build(),
            super_rets.rets().index(0).build(),
            SubtypingVariance::Covariant,
        ),
        SubtypingReasoning::new(
            sub_arg_0.args().index(0).build(),
            super_arg_0.args().index(0).build(),
            SubtypingVariance::Contravariant,
        ),
        SubtypingReasoning::new(
            sub_arg_1.args().index(1).build(),
            super_arg_variadic.args().tail().variadic().build(),
            SubtypingVariance::Contravariant,
        ),
    ];

    assert_eq!(expected.len(), result.reasoning().size());
    for expected_reasoning in expected {
        assert!(result.reasoning().contains(&expected_reasoning));
    }
}
