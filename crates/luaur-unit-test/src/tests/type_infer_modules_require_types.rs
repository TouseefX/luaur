//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_require_types() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_id::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("workspace/A"),
        String::from(
            r#"
        export type Point = {x: number, y: number}

        return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("workspace/B"),
        String::from(
            r#"
        local Hooty = require(workspace.A)

        local h: Hooty.Point
    "#,
        ),
    );

    let b_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("workspace/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("workspace/B"));
    let h_type = fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("h"));
    assert!(
        !unsafe { get_type_id::<TableType>(h_type) }.is_null(),
        "Expected table but got {}",
        to_string_type_id(h_type)
    );
}
