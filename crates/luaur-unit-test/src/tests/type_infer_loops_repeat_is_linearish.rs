//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1490:type_infer_loops_repeat_is_linearish`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_loops_repeat_is_linearish

#[cfg(test)]
#[test]
fn type_infer_loops_repeat_is_linearish() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x = nil
        if math.random () > 0.5 then
            x = ""
            repeat
                error("spooky scary error")
            until true
        end
        -- The repeat in the above branch unconditionally fires the error, so
        -- this should _always_ be `nil`
        local y = x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "nil",
        to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
}
