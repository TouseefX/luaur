use luaur_analysis::enums::pack_field::PackField;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::records::path::Path;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_append() {
    let p = Path::default();
    assert!(p.append(&Path::default()).path_empty());

    let p1 = Path::default();
    let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
    let result = p1.append(&p2);
    assert_eq!(
        result,
        Path::from_component(Component::TypeField(TypeField::Metatable))
    );

    let p1 = Path::from_component(Component::TypeField(TypeField::IndexLookup));
    let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
    let result = p1.append(&p2);
    assert_eq!(
        result,
        Path::from_components(vec![
            Component::TypeField(TypeField::IndexLookup),
            Component::TypeField(TypeField::Metatable),
        ])
    );

    let p1 = Path::from_components(vec![
        Component::TypeField(TypeField::IndexLookup),
        Component::TypeField(TypeField::Metatable),
    ]);
    let p2 = Path::from_components(vec![
        Component::TypeField(TypeField::Metatable),
        Component::PackField(PackField::Arguments),
    ]);
    let result = p1.append(&p2);
    assert_eq!(
        result,
        Path::from_components(vec![
            Component::TypeField(TypeField::IndexLookup),
            Component::TypeField(TypeField::Metatable),
            Component::TypeField(TypeField::Metatable),
            Component::PackField(PackField::Arguments),
        ])
    );

    let p1 = Path::from_component(Component::TypeField(TypeField::IndexLookup));
    let p2 = Path::from_component(Component::TypeField(TypeField::Metatable));
    let _ = p1.append(&p2);
    assert_eq!(
        p1,
        Path::from_component(Component::TypeField(TypeField::IndexLookup))
    );
    assert_eq!(
        p2,
        Path::from_component(Component::TypeField(TypeField::Metatable))
    );
}
