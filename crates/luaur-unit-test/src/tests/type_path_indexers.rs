use crate::records::fixture::Fixture;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_indexers() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
                type T = { [string]: boolean }
            "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let string_type = fixture.get_builtins().stringType;
    let boolean_type = fixture.get_builtins().booleanType;
    let builtins = fixture.get_builtins() as *mut _;

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::IndexLookup)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(string_type)
    );

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::IndexResult)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(boolean_type)
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
                type T = { y: number }
            "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::IndexLookup)),
            unsafe { &*builtins },
            &mut arena,
        ),
        None
    );

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::IndexResult)),
            unsafe { &*builtins },
            &mut arena,
        ),
        None
    );
}
