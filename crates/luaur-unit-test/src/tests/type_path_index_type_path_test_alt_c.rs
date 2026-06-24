//! Node: `cxx:Test:Luau.UnitTest:tests/TypePath.test.cpp:649:index`

use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_index() {
    let mut expected = PathBuilder::new();
    let mut actual = PathBuilder::new();
    assert_eq!(actual.index(0).build(), expected.index(0).build());
}
