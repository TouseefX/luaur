//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1016:type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level

#[cfg(test)]
#[test]
fn type_infer_free_types_introduced_within_control_flow_constructs_do_not_get_an_elevated_type_level(
) {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        if _ then
            _[_], _ = nil
            _()
        end

        local aaa = function():typeof(_) return 1 end

        if aaa then
            while _() do
            end
        end
    "#,
        ),
        None,
    );
}
