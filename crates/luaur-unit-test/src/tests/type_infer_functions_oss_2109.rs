//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3722:type_infer_functions_oss_2109`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_functions_oss_2109

#[cfg(test)]
#[test]
fn type_infer_functions_oss_2109() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function Retry<T..., K...>(
            MaxRetries: number,
            RetryInterval: number,
            Function: (T...) -> (K...),
            ...: T...
        ): K...
            local Results
            local CurrentRetry = 0

            repeat
                Results = {pcall(Function, ...)}

                if not Results[1] then
                    CurrentRetry += 1
                end
            until Results[1] or CurrentRetry == MaxRetries

            return unpack(Results :: any, 2)
        end

        local function Test(a: number, b: number): number
            return a + b
        end

        local a = Retry(5, 1, Test, 5, 10)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
}
