//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:232:type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs

#[cfg(test)]
#[test]
fn type_infer_operators_typecheck_overloaded_multiply_that_is_an_intersection_on_rhs() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local Vec3 = {}
        Vec3.__index = Vec3
        function Vec3.new()
            return setmetatable({x=0, y=0, z=0}, Vec3)
        end

        export type Vec3 = typeof(Vec3.new())

        local thefun: any = function(self, o) return self end

        local multiply: ((Vec3, Vec3) -> Vec3) & ((Vec3, number) -> Vec3) = thefun

        Vec3.__mul = multiply

        local a = Vec3.new()
        local b = Vec3.new()
        local c = b * a
        local d = 2 * a
        local e = 'cabbage' * a
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "Vec3",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "Vec3",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "Vec3",
        to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(
        "Vec3",
        to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "mul<string, Vec3>"
    } else {
        "Vec3"
    };
    assert_eq!(
        expected,
        to_string_type_id(fixture.base.require_type_string(&String::from("e")))
    );
}
