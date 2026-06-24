//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2717:type_infer_refinements_cannot_call_a_function_union`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_refinements_cannot_call_a_function_union

#[cfg(test)]
#[test]
fn type_infer_refinements_cannot_call_a_function_union() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = RefinementExternTypeFixture {
        base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Disconnectable = {
            Disconnect: (self: Disconnectable) -> (...any);
        } | {
            disconnect: (self: Disconnectable) -> (...any)
        } | ExternScriptConnection

        local x: Disconnectable = workspace.ChildAdded:Connect(function()
            print("child added")
        end)

        if type(x.Disconnect) == "function" then
            x:Disconnect()
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    // FIXME CLI-157125: It's a bit clowny that we return a union of
    // functions containing `function` here, but it looks like a side
    // effect of how we execute `hasProp`.
    let expected_error = String::from("Cannot call a value of type function in union:\n")
        + "  ((ExternScriptConnection) -> ()) | function | t2 where t1 = ExternScriptConnection | { Disconnect: t2 } | { "
        + "disconnect: (t1) -> (...any) } ; t2 = (t1) -> (...any)";

    assert_eq!(to_string_type_error(&result.errors[1]), expected_error);
}
