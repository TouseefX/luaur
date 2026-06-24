//! Node: `cxx:Test:Luau.UnitTest:tests/TypePath.test.cpp:678:pack_slice`

use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;

#[test]
fn type_path_pack_slice() {
    let mut expected = PathBuilder::new();
    let mut actual = PathBuilder::new();
    assert_eq!(actual.pack_slice(3).build(), expected.pack_slice(3).build());
}
