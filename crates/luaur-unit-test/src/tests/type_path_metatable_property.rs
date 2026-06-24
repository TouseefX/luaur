use crate::records::builtins_fixture::BuiltinsFixture;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::property_type_path::Property;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

fn check_case(source: &str) {
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
        .base
        .check_string_optional_frontend_options(&String::from(source), None);
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.base.require_type_string(&String::from("x"));
    let number_type = fixture.base.get_builtins().numberType;
    let builtins = fixture.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    let path = Path::from_component(Component::Property(Property::property_string_bool(
        "x", true,
    )));

    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(number_type)
    );
}

#[test]
fn type_path_metatable_property() {
    check_case(
        r#"
            local x = setmetatable({ x = 123 }, {})
        "#,
    );

    check_case(
        r#"
            local x = setmetatable({ x = 123 }, { __index = { x = 'foo' } })
        "#,
    );

    check_case(
        r#"
            local x = setmetatable({}, { __index = { x = 123 } })
        "#,
    );
}
