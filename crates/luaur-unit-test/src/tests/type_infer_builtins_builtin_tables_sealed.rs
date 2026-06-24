//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:201:type_infer_builtins_builtin_tables_sealed`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_builtins_builtin_tables_sealed

#[cfg(test)]
#[test]
fn type_infer_builtins_builtin_tables_sealed() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local b = bit32
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let bit32 = fixture.base.require_type_string(&String::from("b"));
    assert!(!bit32.is_null());
    let bit32t =
        unsafe { get_type_id::<TableType>(bit32).as_ref() }.expect("expected bit32 table type");
    assert_eq!(TableState::Sealed, bit32t.state);
}
