//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1981:type_infer_tables_key_setting_inference_given_nil_upper_bound`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_tables_key_setting_inference_given_nil_upper_bound

#[cfg(test)]
#[test]
fn type_infer_tables_key_setting_inference_given_nil_upper_bound() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function setkey_object(t: { [string]: number }, v)
            t.foo = v
            t.foo = nil
        end
        local function setkey_constindex(t: { [string]: number }, v)
            t["foo"] = v
            t["foo"] = nil
        end
        local function setkey_unknown(t: { [string]: number }, k, v)
            t[k] = v
            t[k] = nil
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "({ [string]: number }, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("setkey_object")))
    );
    assert_eq!(
        "({ [string]: number }, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("setkey_constindex")))
    );
    assert_eq!(
        "({ [string]: number }, string, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("setkey_unknown")))
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function on_number(v: number): () end
        local function setkey_object(t: { [string]: number }, v)
            t.foo = v
            on_number(v)
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "({ [string]: number }, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("setkey_object")))
    );
}
