//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1115:type_infer_extern_types_extern_type_with_indexer_intersect_table`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_extern_type_with_indexer_intersect_table

#[cfg(test)]
#[test]
fn type_infer_extern_types_extern_type_with_indexer_intersect_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
        &String::from(
            r#"
        declare extern type Foobar with
            [string]: unknown
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function update(obj: Foobar)
            assert(typeof(obj.Baz) == "number")
            return obj
        end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "(Foobar) -> Foobar & { read Baz: number }",
        to_string_type_id(fixture.base.require_type_string(&String::from("update")))
    );
}
