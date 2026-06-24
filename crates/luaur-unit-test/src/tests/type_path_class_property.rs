use crate::records::extern_type_fixture::ExternTypeFixture;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::property_type_path::Property;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_class_property() {
    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let root = fixture.vector2_instance_type;
    let number_type = fixture.base.base.get_builtins().numberType;
    let builtins = fixture.base.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    let path = Path::from_component(Component::Property(Property::property_string_bool(
        "X", true,
    )));

    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(number_type)
    );
}
