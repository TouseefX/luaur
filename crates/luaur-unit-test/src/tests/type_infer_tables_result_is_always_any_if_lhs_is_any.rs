//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1280:type_infer_tables_result_is_always_any_if_lhs_is_any`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_tables_result_is_always_any_if_lhs_is_any

#[cfg(test)]
#[test]
fn type_infer_tables_result_is_always_any_if_lhs_is_any() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Vector3MT = {
            __add: (Vector3MT, Vector3MT) -> Vector3MT,
            __mul: (Vector3MT, Vector3MT|number) -> Vector3MT
        }

        local Vector3: {new: (number?, number?, number?) -> Vector3MT}
        local Vector3MT: Vector3MT
        setmetatable(Vector3, Vector3MT)

        type CFrameMT = {
            __mul: (CFrameMT, Vector3MT|CFrameMT) -> Vector3MT|CFrameMT
        }

        local CFrame: {
            Angles:(number, number, number) -> CFrameMT
        }
        local CFrameMT: CFrameMT
        setmetatable(CFrame, CFrameMT)

        local n: any
        local a = (n + Vector3.new(0, 1.5, 0)) * CFrame.Angles(0, math.pi/2, 0)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "any",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
}
