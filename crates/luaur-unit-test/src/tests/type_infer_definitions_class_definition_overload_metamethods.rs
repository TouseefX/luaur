//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:421:type_infer_definitions_class_definition_overload_metamethods`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_definitions_class_definition_overload_metamethods

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definition_overload_metamethods() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Vector3
        end

        declare class CFrame
            function __mul(self, other: CFrame): CFrame
            function __mul(self, other: Vector3): Vector3
        end

        declare function newVector3(): Vector3
        declare function newCFrame(): CFrame
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local base = newCFrame()
        local shouldBeCFrame = base * newCFrame()
        local shouldBeVector = base * newVector3()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "CFrame",
        to_string_type_id(fixture.require_type_string(&String::from("shouldBeCFrame")))
    );
    assert_eq!(
        "Vector3",
        to_string_type_id(fixture.require_type_string(&String::from("shouldBeVector")))
    );
}
