//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2637:type_function_user_udtf_type_alias_registration_follows`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_function_user_udtf_type_alias_registration_follows

#[cfg(test)]
#[test]
fn type_function_user_udtf_type_alias_registration_follows() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
export type t110 = ""type--"
function _<t32...,t0...,t0...,t0...>(...):(any)&(any)
end
if _ then
else
    export type t110 = ""type--"
    function _<t32...,t0...,t0...,t0...>(...):(any)&(any)
    end
end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
