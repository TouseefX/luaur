//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:114:type_infer_tables_augment_nested_table`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tables_augment_nested_table

#[cfg(test)]
#[test]
fn type_infer_tables_augment_nested_table() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = { p = {} }
        t.p.foo = 'bar'
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = unsafe {
        get_type_id::<TableType>(fixture.require_type_string(&String::from("t"))).as_ref()
    }
    .expect("expected table type");
    let p_ty = t_type
        .props
        .get("p")
        .and_then(|prop| prop.read_ty)
        .expect("expected p read type");
    assert!(
        unsafe { !get_type_id::<TableType>(p_ty).is_null() },
        "expected p to have table type"
    );

    let mut opts = ToStringOptions::to_string_options(true);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "{ p: { foo: string } }"
    } else {
        "{| p: {| foo: string |} |}"
    };
    assert_eq!(
        expected,
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("t")),
            &mut opts
        )
    );
}
