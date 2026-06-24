use luaur_analysis::functions::to_string_type_path::to_string;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_chain() {
    let mut builder = PathBuilder::new();
    assert_eq!(
        to_string(&builder.index(0).mt().build(), false),
        "[0].metatable()"
    );
}
