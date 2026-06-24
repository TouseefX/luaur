//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1882:type_infer_tables_type_mismatch_on_massive_table_is_cut_short`
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
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_tables_type_mismatch_on_massive_table_is_cut_short

#[cfg(test)]
#[test]
fn type_infer_tables_type_mismatch_on_massive_table_is_cut_short() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FInt;

    let _sfis = ScopedFastInt::new(&FInt::LuauTableTypeMaximumStringifierLength, 40);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: {a: number,b: number, c: number, d: number, e: number, f: number} = nil :: any
        t = 1
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

    assert_eq!(
        "{ a: number, b: number, c: number, d: number, e: number, ... 1 more ... }",
        to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
    assert_eq!("number", to_string_type_id(tm.given_type));
    assert_eq!(
        "Expected this to be '{ a: number, b: number, c: number, d: number, e: number, ... 1 more ... }', but got 'number'",
        to_string_type_error(&result.errors[0])
    );
}
