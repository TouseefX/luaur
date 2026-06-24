//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:950:frontend_trace_requires_in_nonstrict_mode`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_trace_requires_in_nonstrict_mode

#[cfg(test)]
#[test]
fn frontend_trace_requires_in_nonstrict_mode() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        --!nonstrict
        local module = {}

        function module.f(arg: number)
            print('f', arg)
        end

        return module
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        --!nonstrict
        local A = require(script.Parent.A)

        print(A.g(5))       -- Key 'g' not found
        print(A.f('five'))  -- Type mismatch number and string
        print(A.f(5))       -- OK
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), None);

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(5, result.errors[1].location.begin.line);
}
