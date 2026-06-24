//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:512:type_infer_tables_table_param_width_subtyping_3`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_tables_table_param_width_subtyping_3

#[cfg(test)]
#[test]
fn type_infer_tables_table_param_width_subtyping_3() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::missing_properties::MissingProperties;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _missing_properties_as_nil =
        ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}
        T.bar = 'hello'
        function T:method()
            local a = self.baz
        end
        T:method()
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() && FFlag::LuauSubtypingMissingPropertiesAsNil.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            Location {
                begin: Position { line: 6, column: 8 },
                end: Position { line: 6, column: 9 },
            },
            result.errors[0].location
        );

        if !FFlag::DebugLuauForceOldSolver.get() {
            assert_eq!(
                "Expected this to be '{ read baz: unknown }', but got 'T'",
                to_string_type_error(&result.errors[0])
            );
        } else {
            let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
                .expect("expected MissingProperties");
            assert_eq!(1, error.properties().len());
            assert_eq!("baz", error.properties()[0].as_str());
        }
    }
}
