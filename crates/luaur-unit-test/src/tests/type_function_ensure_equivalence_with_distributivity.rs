//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:838:type_function_ensure_equivalence_with_distributivity`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_ensure_equivalence_with_distributivity

#[cfg(test)]
#[test]
fn type_function_ensure_equivalence_with_distributivity() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
        &String::from(
            r#"
        declare class A
            function __mul(self, rhs: unknown): A
        end

        declare class B
            function __mul(self, rhs: unknown): B
        end

        declare class C
            function __mul(self, rhs: unknown): C
        end

        declare class D
            function __mul(self, rhs: unknown): D
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = mul<A | B, C | D>
        type U = mul<A, C> | mul<A, D> | mul<B, C> | mul<B, D>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "A | B",
        to_string_type_id(fixture.base.require_type_alias(&String::from("T")))
    );
    assert_eq!(
        "A | A | B | B",
        to_string_type_id(fixture.base.require_type_alias(&String::from("U")))
    );
}
