use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::enums::subtyping_variance::SubtypingVariance;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::subtyping_reasoning::SubtypingReasoning;
use luaur_analysis::type_aliases::component::Component;

#[cfg(test)]
#[test]
fn subtyping_negation() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;

    let sub_ty = number_ty;
    let super_ty = fixture.negate(number_ty);

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    let expected = SubtypingReasoning::new(
        Path::default(),
        Path::from_component(Component::TypeField(TypeField::Negated)),
        SubtypingVariance::Covariant,
    );
    assert_eq!(1, result.reasoning().size());
    assert!(result.reasoning().contains(&expected));
}
