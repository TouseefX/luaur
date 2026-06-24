#[cfg(test)]
#[test]
fn type_infer_functions_oss_2125() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type function CombineTableAndSetIndexer(a: type, b: type, c: type)
            local t = {}

            for key, value in a:properties() do
                t[key] = value.read
            end

            if b.tag == "table" then
                for key, value in b:properties() do
                    t[key] = value.read
                end
            end

            return types.newtable(t :: any, { index = types.number, readresult = c, writeresult = c })
        end

        type SpecialProperties = {
            test: string?,
        }

        local function component<Properties>(
            constructor: (props: Properties) -> ()
        ): (
            CombineTableAndSetIndexer<SpecialProperties, Properties, any>
        ) -> ()
            return function(props: Properties) end
        end

        local mrrp = component(function(thing: {
            meow: number,
        }) end)

        mrrp({
            meow = 5,
        })
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
