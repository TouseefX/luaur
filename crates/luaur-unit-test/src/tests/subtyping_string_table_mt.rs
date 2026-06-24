use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;

#[cfg(test)]
#[test]
fn subtyping_string_table_mt() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let sub_ty = string_ty;
    let super_ty = fixture.tbl(SubtypeFixture::props(vec![(
        "X",
        Property::rw_type_id(number_ty),
    )]));

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    let mut sub_path = PathBuilder::new();
    let expected = SubtypingReasoning::new(
        sub_path.mt().read_prop("__index").build(),
        Path::default(),
        SubtypingVariance::Covariant,
    );
    assert_eq!(1, result.reasoning().size());
    assert!(result.reasoning().contains(&expected));
}
