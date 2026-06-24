use crate::records::fixture::Fixture;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type as traverse_for_type_from_type;
use luaur_analysis::functions::traverse_for_type_type_path_alt_b::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_variadic() {
    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let builtins = fixture.get_builtins() as *mut _;

    let mut arena = TypeArena::default();
    let tp = arena.add_type_pack_t(VariadicTypePack::new(number_type));
    assert_eq!(
        traverse_for_type(
            tp,
            &Path::from_component(Component::TypeField(TypeField::Variadic)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(number_type)
    );

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type_from_type(
            number_type,
            &Path::from_component(Component::TypeField(TypeField::Variadic)),
            unsafe { &*builtins },
            &mut arena,
        ),
        None
    );
}
