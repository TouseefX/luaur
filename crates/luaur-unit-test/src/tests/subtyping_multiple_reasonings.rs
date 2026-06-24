use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;

#[cfg(test)]
#[test]
fn subtyping_multiple_reasonings() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let sub_ty = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(string_ty)),
        ("Y", Property::rw_type_id(number_ty)),
    ]));
    let super_ty = fixture.tbl(SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(number_ty)),
        ("Y", Property::rw_type_id(string_ty)),
    ]));

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    let mut sub_x = PathBuilder::new();
    let mut super_x = PathBuilder::new();
    let mut sub_y = PathBuilder::new();
    let mut super_y = PathBuilder::new();
    let expected = [
        SubtypingReasoning::new(
            sub_x.read_prop("X").build(),
            super_x.read_prop("X").build(),
            SubtypingVariance::Invariant,
        ),
        SubtypingReasoning::new(
            sub_y.read_prop("Y").build(),
            super_y.read_prop("Y").build(),
            SubtypingVariance::Invariant,
        ),
    ];

    assert_eq!(expected.len(), result.reasoning().size());
    for expected_reasoning in expected {
        assert!(result.reasoning().contains(&expected_reasoning));
    }
}
