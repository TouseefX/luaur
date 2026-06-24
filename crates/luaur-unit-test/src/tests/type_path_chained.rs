use luaur_analysis::enums::pack_field::PackField;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::property_type_path::Property;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_chained() {
    let mut builder = PathBuilder::new();
    let result = builder
        .index(0)
        .read_prop("foo")
        .mt()
        .read_prop("bar")
        .args()
        .index(1)
        .build();

    let mut index_0 = PathBuilder::new();
    let mut index_1 = PathBuilder::new();
    let expected = Path::from_components(vec![
        index_0.index(0).build().components.remove(0),
        Component::Property(Property::property_string_bool("foo", true)),
        Component::TypeField(TypeField::Metatable),
        Component::Property(Property::property_string_bool("bar", true)),
        Component::PackField(PackField::Arguments),
        index_1.index(1).build().components.remove(0),
    ]);

    assert_eq!(result, expected);
}
