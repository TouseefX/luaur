use crate::records::fixture::Fixture;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::property_type_path::Property;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_table_property() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x = { y = 123 }
    "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_string(&String::from("x"));
    let number_type = fixture.get_builtins().numberType;
    let builtins = fixture.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    let path = Path::from_component(Component::Property(Property::property_string_bool(
        "y", true,
    )));

    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(number_type)
    );
}
