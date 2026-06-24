//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:666:type_infer_loops_unreachable_code_after_infinite_loop`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_loops_unreachable_code_after_infinite_loop

#[cfg(test)]
#[test]
fn type_infer_loops_unreachable_code_after_infinite_loop() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            function unreachablecodepath(a): number
                while true do
                    if a then return 10 end
                end
                -- unreachable
            end
            unreachablecodepath(4)
        "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            function reachablecodepath(a): number
                while true do
                    if a then break end
                    return 10
                end

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
        .expect("expected FunctionExitsWithoutReturning");

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            function unreachablecodepath(a): number
                repeat
                    if a then return 10 end
                until false

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            function reachablecodepath(a, b): number
                repeat
                    if a then break end

                    if b then return 10 end
                until false

                print("x") -- correct error
            end
            reachablecodepath(4)
        "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
        .expect("expected FunctionExitsWithoutReturning");

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            function unreachablecodepath(a: number?): number
                repeat
                    return 10
                until a ~= nil

                -- unreachable
            end
            unreachablecodepath(4)
        "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
