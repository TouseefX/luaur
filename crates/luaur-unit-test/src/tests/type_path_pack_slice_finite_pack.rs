use crate::records::fixture::Fixture;
use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use luaur_analysis::functions::traverse_for_pack_type_path::traverse_for_pack;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::type_arena::TypeArena;

#[test]
fn type_path_pack_slice_finite_pack() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = (number, string) -> ()
    "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;
    let mut builder = PathBuilder::new();
    let path = builder.args().pack_slice(1).build();
    let mut arena = TypeArena::default();
    let result = traverse_for_pack(root, &path, unsafe { &*builtins }, &mut arena);
    assert!(result.is_some());
    assert_eq!(to_string_type_pack_id(result.unwrap()), "string");
}
