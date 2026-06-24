use crate::records::builtins_fixture::BuiltinsFixture;
use crate::records::extern_type_fixture::ExternTypeFixture;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_metatables() {
    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let builtins: *mut BuiltinTypes = fixture.base.base.get_builtins() as *mut _;
    let string_type = unsafe { (*builtins).stringType };
    let string_metatable =
        get_metatable_type_id_not_null_builtin_types(string_type, unsafe { &*builtins });
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            string_type,
            &Path::from_component(Component::TypeField(TypeField::Metatable)),
            unsafe { &*builtins },
            &mut arena,
        ),
        string_metatable
    );

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            type T = "foo"
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let root = fixture.base.base.require_type_alias(&String::from("T"));
    let builtins: *mut BuiltinTypes = fixture.base.base.get_builtins() as *mut _;
    let string_metatable =
        get_metatable_type_id_not_null_builtin_types(unsafe { (*builtins).stringType }, unsafe {
            &*builtins
        });
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::Metatable)),
            unsafe { &*builtins },
            &mut arena,
        ),
        string_metatable
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            type Table = { foo: number }
            type Metatable = { bar: number }
            local tbl: Table = { foo = 123 }
            local mt: Metatable = { bar = 456 }
            local res = setmetatable(tbl, mt)
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let root = fixture.base.require_type_string(&String::from("res"));
    let expected = fixture.base.lookup_type(&String::from("Table")).unwrap();
    let builtins = fixture.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::Table)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(unsafe { follow_type_id(expected) })
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            local mt = { foo = 123 }
            local tbl = setmetatable({}, mt)
        "#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let root = fixture.base.require_type_string(&String::from("tbl"));
    let expected = fixture.base.require_type_string(&String::from("mt"));
    let builtins = fixture.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    assert_eq!(
        traverse_for_type(
            root,
            &Path::from_component(Component::TypeField(TypeField::Metatable)),
            unsafe { &*builtins },
            &mut arena,
        ),
        Some(expected)
    );

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let builtins = fixture.base.base.get_builtins() as *mut _;
    let mut arena = TypeArena::default();
    assert!(traverse_for_type(
        fixture.vector2_instance_type,
        &Path::from_component(Component::TypeField(TypeField::Metatable)),
        unsafe { &*builtins },
        &mut arena,
    )
    .is_some());
}
