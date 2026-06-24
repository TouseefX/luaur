//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:463:non_strict_type_checker_generic_type_instantiation`
//! Source: `tests/NonStrictTypeChecker.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_generic_type_instantiation

#[cfg(test)]
#[test]
fn non_strict_type_checker_generic_type_instantiation() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_common::FFlag;

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
        function array<T>(): {T}
            return {}
        end

        local foo = array<<number>>()
        local bar = array<<string>>()
    "#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "{number}",
        to_string_type_id(fixture.base.require_type_string(&String::from("foo")))
    );
    assert_eq!(
        "{string}",
        to_string_type_id(fixture.base.require_type_string(&String::from("bar")))
    );
}
