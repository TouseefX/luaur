//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3415:type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back

#[cfg(test)]
#[test]
fn type_infer_tables_dont_invalidate_the_properties_iterator_of_free_table_when_rolled_back() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.file_resolver.source.insert(
        String::from("Module/Backend/Types"),
        String::from(
            r#"
        export type Fiber = {
            return_: Fiber?
        }
        return {}
    "#,
        ),
    );

    fixture.file_resolver.source.insert(
        String::from("Module/Backend"),
        String::from(
            r#"
        local Types = require(script.Types)
        type Fiber = Types.Fiber
        type ReactRenderer = { findFiberByHostInstance: () -> Fiber? }

        local function attach(renderer): ()
            local function getPrimaryFiber(fiber)
                local alternate = fiber.alternate
                return fiber
            end

            local function getFiberIDForNative()
                local fiber = renderer.findFiberByHostInstance()
                fiber = fiber.return_
                return getPrimaryFiber(fiber)
            end
        end

        function culprit(renderer: ReactRenderer): ()
            attach(renderer)
        end

        return culprit
    "#,
        ),
    );

    let _result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/Backend"), None);
}
