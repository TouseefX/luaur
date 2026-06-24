//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1578:type_infer_lti_must_record_contributing_locations`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_lti_must_record_contributing_locations

#[cfg(test)]
#[test]
fn type_infer_lti_must_record_contributing_locations() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(a)
            if math.random() > 0.5 then
                math.abs(a)
            else
                string.len(a)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let fn_ty = fixture.base.require_type_string(&String::from("f"));
    let function = unsafe { get_type_id::<FunctionType>(fn_ty).as_ref() }
        .expect("expected f to have a function type");

    let arg_ty = first(function.arg_types(), false).expect("expected first argument");
    let module = unsafe { &*fixture.base.get_main_module(false) };
    let locations = module
        .upper_bound_contributors
        .find(&arg_ty)
        .expect("expected upper-bound contributors for f argument");
    assert_eq!(2, locations.len());
}
