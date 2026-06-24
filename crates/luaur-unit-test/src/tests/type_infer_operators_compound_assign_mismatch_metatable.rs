//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:522:type_infer_operators_compound_assign_mismatch_metatable`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_operators_compound_assign_mismatch_metatable

#[cfg(test)]
#[test]
fn type_infer_operators_compound_assign_mismatch_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type V2B = { x: number, y: number }
        local v2b: V2B = { x = 0, y = 0 }
        local VMT = {}
        type V2 = typeof(setmetatable(v2b, VMT))

        function VMT.__mod(a: V2, b: V2): number
            return a.x * b.x + a.y * b.y
        end

        local v1: V2 = setmetatable({ x = 1, y = 2 }, VMT)
        local v2: V2 = setmetatable({ x = 3, y = 4 }, VMT)
        v1 %= v2
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be 'V2', but got 'number'",
        to_string_type_error(&result.errors[0])
    );
}
