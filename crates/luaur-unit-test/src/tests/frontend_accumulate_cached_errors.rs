//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:772:frontend_accumulate_cached_errors`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_accumulate_cached_errors

#[cfg(test)]
#[test]
fn frontend_accumulate_cached_errors() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            r#"
        local n: number = 'five'
        return {n=n}
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/B"),
        String::from(
            r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'seven'
        print(A, b)
    "#,
        ),
    );

    let result1 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

    assert_eq!("Modules/A", result1.errors[0].module_name);
    assert_eq!("Modules/B", result1.errors[1].module_name);

    let result2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(2, result2.errors.len(), "{:?}", result2.errors);

    assert_eq!("Modules/A", result2.errors[0].module_name);
    assert_eq!("Modules/B", result2.errors[1].module_name);
}
