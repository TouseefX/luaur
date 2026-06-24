//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1465:type_infer_operators_luau_polyfill_is_array`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_operators_luau_polyfill_is_array

#[cfg(test)]
#[test]
fn type_infer_operators_luau_polyfill_is_array() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
return function(value: any): boolean
    if typeof(value) ~= "table" then
        return false
    end
    if next(value) == nil then
        -- an empty table is an empty array
        return true
    end

    local length = #value

    if length == 0 then
        return false
    end

    local count = 0
    local sum = 0
    for key in pairs(value) do
        if typeof(key) ~= "number" then
            return false
        end
        if key % 1 ~= 0 or key < 1 then
            return false
        end
        count += 1
        sum += key
    end

    return sum == (count * (count + 1) / 2)
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
