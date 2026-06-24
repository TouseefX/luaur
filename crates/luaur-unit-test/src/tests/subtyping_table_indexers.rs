use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;
use luaur_analysis::type_aliases::component::Component;

#[cfg(test)]
#[test]
fn subtyping_table_indexers() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let sub_ty = fixture.idx(number_ty, string_ty);
    let super_ty = fixture.idx(string_ty, number_ty);

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    let expected = [
        SubtypingReasoning::new(
            Path::from_component(Component::TypeField(TypeField::IndexLookup)),
            Path::from_component(Component::TypeField(TypeField::IndexLookup)),
            SubtypingVariance::Invariant,
        ),
        SubtypingReasoning::new(
            Path::from_component(Component::TypeField(TypeField::IndexResult)),
            Path::from_component(Component::TypeField(TypeField::IndexResult)),
            SubtypingVariance::Invariant,
        ),
    ];
    assert_eq!(expected.len(), result.reasoning().size());
    for expected_reasoning in expected {
        assert!(result.reasoning().contains(&expected_reasoning));
    }
}
