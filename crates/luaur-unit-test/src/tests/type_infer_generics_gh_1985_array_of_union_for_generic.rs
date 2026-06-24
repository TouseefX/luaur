//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2047:type_infer_generics_gh_1985_array_of_union_for_generic`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - translates_to -> rust_item type_infer_generics_gh_1985_array_of_union_for_generic

#[cfg(test)]
#[test]
fn type_infer_generics_gh_1985_array_of_union_for_generic() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function clear<T>(arr: { T }) table.clear(arr) end
        local a: { true | false }
        -- This obviously shouldn't error, '{ true | false }' should fit '{ T }'
        -- TypeError: The generic type parameter Twas found to have invalid bounds. Its lower bounds were [true, false], and its upper bounds were [true].
        clear(a)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
