use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;

#[cfg(test)]
#[test]
fn subtyping_table_property() {
    let mut fixture = SubtypeFixture::default();
    let boolean_ty = fixture.builtin_types.booleanType;
    let number_ty = fixture.builtin_types.numberType;

    let sub_ty = fixture.tbl(SubtypeFixture::props(vec![(
        "X",
        Property::rw_type_id(number_ty),
    )]));
    let super_ty = fixture.tbl(SubtypeFixture::props(vec![(
        "X",
        Property::rw_type_id(boolean_ty),
    )]));

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());
    assert_eq!(1, result.reasoning().size());

    let mut sub_path = PathBuilder::new();
    let mut super_path = PathBuilder::new();
    let expected = SubtypingReasoning::new(
        sub_path.read_prop("X").build(),
        super_path.read_prop("X").build(),
        SubtypingVariance::Invariant,
    );
    assert!(result.reasoning().contains(&expected));
}
