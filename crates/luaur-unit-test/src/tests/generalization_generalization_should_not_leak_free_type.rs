//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:339:generalization_generalization_should_not_leak_free_type`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item generalization_generalization_should_not_leak_free_type

#[cfg(test)]
#[test]
fn generalization_generalization_should_not_leak_free_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _flag = ScopedFastFlag::new(&FFlag::DebugLuauForbidInternalTypes, true);
    let mut fixture = BuiltinsFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo()

            local productButtonPairs = {}
            local func
            local dir = -1

            local function updateSearch()
                for product, button in pairs(productButtonPairs) do
                    -- This line may have a floating free type pack.
                    button.LayoutOrder = func(product) * dir
                end
            end

            function(mode)
                if mode == 'New'then
                    func = function(p)
                        return p.id
                    end
                elseif mode == 'Price'then
                    func = function(p)
                        return p.price
                    end
                end
            end
        end
    "#,
        ),
        None,
    );
}
