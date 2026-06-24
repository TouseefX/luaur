//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1143:type_infer_tables_meta_add_inferred`
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
//!   - translates_to -> rust_item type_infer_tables_meta_add_inferred

#[cfg(test)]
#[test]
fn type_infer_tables_meta_add_inferred() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = {}
        setmetatable(a, {__add=function(a,b) return b end} )
        local c = a + a
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        unsafe { follow_type_id(fixture.base.require_type_string(&String::from("a"))) },
        unsafe { follow_type_id(fixture.base.require_type_string(&String::from("c"))) }
    );
}
