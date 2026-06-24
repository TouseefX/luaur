//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4217:type_infer_tables_cli_84607_missing_prop_in_array_or_dict`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_cli_84607_missing_prop_in_array_or_dict

#[cfg(test)]
#[test]
fn type_infer_tables_cli_84607_missing_prop_in_array_or_dict() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::missing_properties::MissingProperties;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _fix = ScopedFastFlag::new(&FFlag::LuauFixIndexerSubtypingOrdering, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Thing = { name: string, prop: boolean }

        local arrayOfThings : {Thing} = {
            { name = "a" }
        }

        local dictOfThings : {[string]: Thing} = {
            a = { name = "a" }
        }
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        for error in &result.errors {
            let error = type_error_data_ref::<MissingProperties>(error)
                .expect("expected MissingProperties");
            assert_eq!(1, error.properties().len());
            assert_eq!("prop", error.properties()[0].as_str());
        }
    } else {
        let error1 = type_error_data_ref::<MissingProperties>(&result.errors[0])
            .expect("expected MissingProperties");
        assert_eq!(1, error1.properties().len());
        assert_eq!("prop", error1.properties()[0].as_str());

        let mismatch =
            type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
        let nested = mismatch.error.as_ref().expect("expected nested TypeError");
        let error2 =
            type_error_data_ref::<MissingProperties>(nested).expect("expected MissingProperties");
        assert_eq!(1, error2.properties().len());
        assert_eq!("prop", error2.properties()[0].as_str());
    }
}
