//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_general_require_multi_assign() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("workspace/A"),
        String::from(
            r#"
        export type myvec2 = {x: number, y: number}
        return {}
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("workspace/B"),
        String::from(
            r#"
        export type myvec3 = {x: number, y: number, z: number}
        return {}
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("workspace/C"),
        String::from(
            r#"
        local Foo, Bar = require(workspace.A), require(workspace.B)

        local a: Foo.myvec2
        local b: Bar.myvec3
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("workspace/C"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a_type_id = fixture
        .base
        .require_type_module_name_string("workspace/C", &String::from("a"));
    let a_type = unsafe { get_type_id::<TableType>(follow_type_id(a_type_id)).as_ref() }
        .expect("expected table type for a");
    assert_eq!(2, a_type.props.len());

    let b_type_id = fixture
        .base
        .require_type_module_name_string("workspace/C", &String::from("b"));
    let b_type = unsafe { get_type_id::<TableType>(follow_type_id(b_type_id)).as_ref() }
        .expect("expected table type for b");
    assert_eq!(3, b_type.props.len());
}
