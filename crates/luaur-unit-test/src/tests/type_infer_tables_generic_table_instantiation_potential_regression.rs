//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3872:type_infer_tables_generic_table_instantiation_potential_regression`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_generic_table_instantiation_potential_regression

#[cfg(test)]
#[test]
fn type_infer_tables_generic_table_instantiation_potential_regression() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::missing_properties::MissingProperties;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict

function f(x)
  x.p = 5
  return x
end
local g : ({ p : number, q : string }) -> ({ p : number, r : boolean }) = f
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let error =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
        assert_eq!(
            "({ p: number, q: string }) -> { p: number, r: boolean }",
            to_string_type_id(error.wanted_type)
        );
        assert_eq!(
            "({ p: number }) -> { p: number }",
            to_string_type_id(error.given_type)
        );
    } else {
        let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
            .expect("expected MissingProperties");
        assert_eq!(1, error.properties().len());
        assert_eq!("r", error.properties()[0].as_str());
    }
}
