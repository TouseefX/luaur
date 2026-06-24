use crate::records::builtins_fixture::BuiltinsFixture;
use crate::records::fixture::Fixture;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::methods::path_builder_rets::PathBuilderRets;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::type_arena::TypeArena;

#[test]
fn type_path_complex_chains() {
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            type Meta = {
                __add: (Tab, Tab) -> number,
            }

            type Tab = typeof(setmetatable({}, {} :: Meta))
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.base.require_type_alias(&String::from("Tab"));
    let number_type = fixture.base.get_builtins().numberType;
    let builtins = fixture.base.get_builtins() as *mut _;
    let mut builder = PathBuilder::new();
    let path = builder.mt().read_prop("__add").rets().index(0).build();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(number_type)
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
            type Obj = {
                method: ((true, false) -> string) & ((string) -> number)
            }
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("Obj"));
    let false_type = fixture.get_builtins().falseType;
    let builtins = fixture.get_builtins() as *mut _;
    let mut builder = PathBuilder::new();
    let path = builder.read_prop("method").index(0).args().index(1).build();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(false_type)
    );
}
