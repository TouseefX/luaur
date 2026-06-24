//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1153:type_infer_extern_types_extern_type_overload`
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
//!   - translates_to -> rust_item type_infer_extern_types_extern_type_overload

#[cfg(test)]
#[test]
fn type_infer_extern_types_extern_type_overload() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

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
        local f : ((Color3) -> ()) & (({Color3}) -> ())
        local c: Color3
        f(c)
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
