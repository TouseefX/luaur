use crate::records::fixture::Fixture;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::negation_type::NegationType;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_negated() {
    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let builtins = fixture.get_builtins() as *mut _;

    let mut arena = TypeArena::default();
    let ty = arena.add_type(NegationType::new(number_type));
    assert_eq!(
        traverse_for_type(
            ty,
            &Path::from_component(Component::TypeField(TypeField::Negated)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(number_type)
    );

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            number_type,
            &Path::from_component(Component::TypeField(TypeField::Negated)),
            unsafe { &*builtins },
            &mut arena,
        ),
        None
    );
}
