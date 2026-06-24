//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:380:type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations

#[cfg(test)]
#[test]
fn type_infer_unknownnever_lti_error_at_declaration_for_never_normalizations() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function num(x: number) end
        local function str(x: string) end
        local function cond(): boolean return false end

        local function f(a)
            if cond() then
                num(a)
            else
                str(a)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Parameter 'a' has been reduced to never. This function is not callable with any possible value.",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Parameter 'a' is required to be a subtype of 'number' here.",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "Parameter 'a' is required to be a subtype of 'string' here.",
        to_string_type_error(&result.errors[2])
    );
}
