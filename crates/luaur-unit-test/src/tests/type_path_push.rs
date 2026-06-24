use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::records::path::Path;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_push() {
    let p = Path::default();
    let result = p.push(Component::TypeField(TypeField::Metatable));

    assert!(p.path_empty());
    assert_eq!(
        result,
        Path::from_component(Component::TypeField(TypeField::Metatable))
    );
}
