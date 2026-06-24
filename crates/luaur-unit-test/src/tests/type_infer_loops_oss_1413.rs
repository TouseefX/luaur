//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1359:type_infer_loops_oss_1413`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_loops_oss_1413

#[cfg(test)]
#[test]
fn type_infer_loops_oss_1413() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function KahanSum(values: {number}): number
            local sum: number = 0
            local compensator: number = 0
            for _, value in values do
                local y = value - compensator
                local t = sum + y
                compensator = (t - sum) - y
                sum = t
            end
            return sum
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function HistogramString(values: {number})
            local histogram = {}
            values = table.clone(values)
            table.sort(values)

            local count = #values
            local range = (count - 1)

            local digitIndex = range // 2 + 1
            while digitIndex < count and values[digitIndex] == 0 do
                digitIndex = count - ((count - digitIndex) // 2)
            end
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function fun1()
            local foo = 1
            local bar = foo - foo + foo
            while false do
                foo = bar
            end
        end
        local function fun2()
            local foo = 1
            while false do
                local bar = foo - foo + foo
                foo = bar
            end
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
