use crate::records::fixture::Fixture;
use luaur_analysis::enums::pack_field::PackField;
use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use luaur_analysis::functions::traverse_for_pack_type_path::traverse_for_pack;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_tail() {
    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
            type T = (number, string, ...boolean) -> ()
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let root = fixture.require_type_alias(&String::from("T"));
    let builtins = fixture.get_builtins() as *mut _;
    let path = Path::from_components(vec![
        Component::PackField(PackField::Arguments),
        Component::PackField(PackField::Tail),
    ]);
    let mut arena = TypeArena::default();
    let result = traverse_for_pack(root, &path, unsafe { &*builtins }, &mut arena);
    assert!(result.is_some());
    assert_eq!(to_string_type_pack_id(result.unwrap()), "...boolean");

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
    let string_type = fixture.get_builtins().stringType;
    let builtins = fixture.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_pack(root, &path, unsafe { &*builtins }, &mut arena),
        None
    );

    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_pack(string_type, &path, unsafe { &*builtins }, &mut arena),
        None
    );
}
