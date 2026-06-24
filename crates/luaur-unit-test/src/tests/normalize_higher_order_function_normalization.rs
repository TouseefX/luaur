//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:758:normalize_higher_order_function_normalization`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function isNumber (Analysis/src/Type.cpp)
//!   - translates_to -> rust_item normalize_higher_order_function_normalization

#[cfg(test)]
#[test]
fn normalize_higher_order_function_normalization() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::primitive_type::Type;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function apply(f, x)
            return f(x)
        end

        local a = apply(function(x: number) return x + x end, 5)
    "#,
        ),
        None,
    );

    let a_type = fixture.require_type_string(&String::from("a"));
    assert_eq!(
        Some(Type::Number),
        fixture.get_primitive_type(a_type),
        "Expected a number but got {}",
        to_string_type_id(a_type)
    );
}
