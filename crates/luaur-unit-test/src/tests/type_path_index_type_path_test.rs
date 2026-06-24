use crate::records::fixture::Fixture;
use luaur_analysis::enums::pack_field::PackField;
use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_index::PathBuilderIndex;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_index() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
            type T = number | string | boolean
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let string_type = fixture.get_builtins().stringType;
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &builder.index(1).build(),
            unsafe { &*builtins },
            &mut arena
        ),
        Some(string_type)
    );

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &builder.index(97).build(),
            unsafe { &*builtins },
            &mut arena
        ),
        None
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
            type T = (() -> ()) & ((true) -> false) & ((false) -> true)
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    let result = traverse_for_type(
        root,
        &builder.index(1).build(),
        unsafe { &*builtins },
        &mut arena,
    );
    assert!(result.is_some());
    assert_eq!(to_string_type_id(result.unwrap()), "(true) -> false");

    let mut builder = PathBuilder::new();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &builder.index(97).build(),
            unsafe { &*builtins },
            &mut arena
        ),
        None
    );

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
            type T = (number, string, true, false) -> ()
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let string_type = fixture.get_builtins().stringType;
    let builtins = fixture.get_builtins() as *mut _;

    let mut builder = PathBuilder::new();
    let path = builder.args().index(1).build();
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        Some(string_type)
    );

    let path = Path::from_components(vec![Component::PackField(PackField::Arguments)]);
    let mut builder = PathBuilder::new();
    let path = path.append(&builder.index(72).build());
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(root, &path, unsafe { &*builtins }, &mut arena),
        None
    );
}
