//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:900:non_strict_type_checker_typecheck_class_method_bodies`
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
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item non_strict_type_checker_typecheck_class_method_bodies

#[cfg(test)]
#[test]
fn non_strict_type_checker_typecheck_class_method_bodies() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::check_result::CheckResult;
    use luaur_analysis::records::checked_function_call_error::CheckedFunctionCallError;
    use luaur_common::FFlag;

    let _force_old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _user_defined_classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy_type_prototyping = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result: CheckResult = fixture.check_non_strict(&String::from(
        r#"
        --!nonstrict
        class Student
            public name: number
            function greet(self)
                return `Hello, {lower(self.name)}!`
            end
        end
    "#,
    ));

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<CheckedFunctionCallError>(&result.errors[0]).is_some());
}
