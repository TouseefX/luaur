use crate::records::fixture::Fixture;
use luaur_analysis::functions::traverse_for_pack_type_path::traverse_for_pack;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::type_arena::TypeArena;

#[test]
fn type_path_pack_slice_type() {
    let mut fixture = Fixture::default();
    let string_type = fixture.get_builtins().stringType;
    let builtins = fixture.get_builtins() as *mut _;
    let mut builder = PathBuilder::new();
    let path = builder.args().pack_slice(1).build();
    let mut arena = TypeArena::default();

    assert_eq!(
        traverse_for_pack(string_type, &path, unsafe { &*builtins }, &mut arena),
        None
    );
}
