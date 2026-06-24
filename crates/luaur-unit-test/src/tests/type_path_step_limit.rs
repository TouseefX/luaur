use crate::records::fixture::Fixture;
use crate::type_aliases::scoped_fast_int::ScopedFastInt;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::type_arena::TypeArena;

#[test]
fn type_path_step_limit() {
    let _sfi = ScopedFastInt::new(&luaur_common::DFInt::LuauTypePathMaximumTraverseSteps, 2);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = {
            x: {
                y: {
                    z: number
                }
            }
        }
    "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;
    let mut builder = PathBuilder::new();
    let path = builder.read_prop("x").read_prop("y").read_prop("z").build();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        None
    );
}
