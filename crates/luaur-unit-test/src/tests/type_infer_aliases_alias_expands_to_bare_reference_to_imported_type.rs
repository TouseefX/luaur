//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_alias_expands_to_bare_reference_to_imported_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!strict
        export type Object = {[string]: any}
        return {}
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local A = require(script.Parent.A)

        type Object = A.Object
        type ReadOnly<T> = T

        local function f(): ReadOnly<Object>
            return nil :: any
        end
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
