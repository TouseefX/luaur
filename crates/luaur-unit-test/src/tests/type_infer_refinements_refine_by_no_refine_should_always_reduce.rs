//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2813:type_infer_refinements_refine_by_no_refine_should_always_reduce`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refine_by_no_refine_should_always_reduce

#[cfg(test)]
#[test]
fn type_infer_refinements_refine_by_no_refine_should_always_reduce() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(t): boolean return true end

        function select<K, V>(t: { [K]: V }, columns: { K }): { [K]: V }
            local result = {}
            if foo(t) then
                for k, v in t do
                    if table.find(columns, k) then
                        result[k] = v -- was TypeError: Type function instance refine<intersect<K, ~nil>, *no-refine*> is uninhabited
                    end
                end
            else
                for k, v in pairs(t) do
                    if table.find(columns, k) then
                        result[k] = v
                    end
                end
            end
            return result
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
