//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2273:type_infer_self_bound_due_to_compound_assign`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_self_bound_due_to_compound_assign

#[cfg(test)]
#[test]
fn type_infer_self_bound_due_to_compound_assign() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.load_definition(
        &String::from(
            r#"
        declare class Camera
            CameraType: string
            CFrame: number
        end
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function MT_UPDATE(CAMERA: Camera, Enum: any, totalOffsets: number, focusToCFrame: number, magnitude: number)
            if CAMERA.CameraType ~= Enum.CameraType.Custom then
                return
            end

            local goalCFrame = (CAMERA.CFrame) * totalOffsets
            if goalCFrame ~= CAMERA.CFrame then
                goalCFrame -= (focusToCFrame * magnitude) -- Offset the goalCFrame the raycast direction based on the cutoff distance.
            end
        end

        return {}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
