use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_empty_path() {
    let mut builder = PathBuilder::new();
    assert!(builder.build().path_empty());
}
