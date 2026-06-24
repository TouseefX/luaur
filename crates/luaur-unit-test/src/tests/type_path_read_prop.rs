use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::property_type_path::Property;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_read_prop() {
    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.read_prop("foo").build(),
        Path::from_component(Component::Property(Property::property_string_bool(
            "foo", true
        )))
    );
}
