//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_do_not_quantify_unresolved_aliases() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local KeyPool = {}

        local function newkey(pool: KeyPool, index)
            return {}
        end

        function newKeyPool()
            local pool = {
                available = {} :: {Key},
            }

            return setmetatable(pool, KeyPool)
        end

        export type KeyPool = typeof(newKeyPool())
        export type Key = typeof(newkey(newKeyPool(), 1))
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
