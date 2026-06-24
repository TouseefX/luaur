use luaur_analysis::functions::to_string_human::to_string_human;
use luaur_analysis::functions::to_string_type_path::to_string;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_pack_slice() {
    let mut string_builder = PathBuilder::new();
    assert_eq!(
        to_string(&string_builder.pack_slice(1).build(), false),
        "[1:]"
    );

    let mut human_builder = PathBuilder::new();
    assert_eq!(
        to_string_human(&human_builder.pack_slice(1).build()),
        "the portion of the type pack starting at index 1 to the end"
    );
}
