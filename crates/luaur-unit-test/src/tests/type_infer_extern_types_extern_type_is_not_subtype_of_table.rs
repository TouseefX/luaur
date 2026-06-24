//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1135:type_infer_extern_types_extern_type_is_not_subtype_of_table`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_extern_types_extern_type_is_not_subtype_of_table

#[cfg(test)]
#[test]
fn type_infer_extern_types_extern_type_is_not_subtype_of_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
        &String::from(
            r#"
        declare extern type Color3 with
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(c: Color3): { Color3 }
            return c
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("Color3", to_string_type_id(err.given_type));
    assert_eq!("{Color3}", to_string_type_id(err.wanted_type));
}
