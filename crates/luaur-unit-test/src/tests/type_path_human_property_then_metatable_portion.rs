use luaur_analysis::functions::to_string_human::to_string_human;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_human_property_then_metatable_portion() {
    let mut read_builder = PathBuilder::new();
    assert_eq!(
        to_string_human(&read_builder.read_prop("a").mt().build()),
        "accessing `a` has the metatable portion as "
    );

    let mut write_builder = PathBuilder::new();
    assert_eq!(
        to_string_human(&write_builder.write_prop("a").mt().build()),
        "writing to `a` has the metatable portion as "
    );
}
