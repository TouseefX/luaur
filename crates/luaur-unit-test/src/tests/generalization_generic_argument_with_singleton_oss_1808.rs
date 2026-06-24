//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:411:generalization_generic_argument_with_singleton_oss_1808`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item generalization_generic_argument_with_singleton_oss_1808

#[cfg(test)]
#[test]
fn generalization_generic_argument_with_singleton_oss_1808() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function test<T>(value: false | (T) -> T)
            return value
        end
        test(false)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
