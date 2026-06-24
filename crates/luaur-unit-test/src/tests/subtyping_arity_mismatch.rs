use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;

#[cfg(test)]
#[test]
fn subtyping_arity_mismatch() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;

    let sub_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![]);
    let super_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    let mut sub_path = PathBuilder::new();
    let mut super_path = PathBuilder::new();
    let expected = SubtypingReasoning::new(
        sub_path.args().build(),
        super_path.args().build(),
        SubtypingVariance::Contravariant,
    );
    assert_eq!(1, result.reasoning().size());
    assert!(result.reasoning().contains(&expected));
}
